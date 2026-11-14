// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact defining identities and closed contracts for source-owned backend bodies.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_parser_chirho::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use haskelujah_runtime_chirho::{
    int_arithmetic_chirho::{IntOperationChirho, IntTypeChirho},
    integer_arithmetic_chirho::{IntegerOperationChirho, IntegerResultChirho},
};
use haskelujah_span_chirho::FileIdChirho;
mod integer_chirho;
#[cfg(test)]
mod tests_chirho;

#[derive(Clone, Copy)]
pub(in crate::next_chirho) enum ProviderChirho {
    IntegerChirho(IntegerOperationChirho),
    IntChirho(IntOperationChirho),
}
impl ProviderChirho {
    pub(in crate::next_chirho) fn primitive_chirho(self) -> &'static str {
        match self {
            Self::IntegerChirho(operation_chirho) => operation_chirho.primitive_chirho(),
            Self::IntChirho(operation_chirho) => operation_chirho.primitive_chirho(),
        }
    }
    pub(in crate::next_chirho) fn contract_chirho(self) -> PrimitiveContractChirho {
        let (arity_chirho, argument_chirho, result_chirho) = match self {
            Self::IntegerChirho(operation_chirho) => (
                operation_chirho.arity_chirho(),
                InterfaceBuiltinChirho::IntegerChirho,
                match operation_chirho.result_chirho() {
                    IntegerResultChirho::IntegerChirho => InterfaceBuiltinChirho::IntegerChirho,
                    IntegerResultChirho::BoolChirho => InterfaceBuiltinChirho::BoolChirho,
                    IntegerResultChirho::OrderingChirho => InterfaceBuiltinChirho::OrderingChirho,
                },
            ),
            Self::IntChirho(operation_chirho) => (
                operation_chirho.arity_chirho(),
                int_type_chirho(operation_chirho.argument_chirho()),
                int_type_chirho(operation_chirho.result_chirho()),
            ),
        };
        let mut ty_chirho = InterfaceTypeChirho::BuiltinChirho(result_chirho);
        for _argument_chirho in 0..arity_chirho {
            ty_chirho = InterfaceTypeChirho::FunChirho(
                Box::new(InterfaceTypeChirho::BuiltinChirho(argument_chirho)),
                Box::new(ty_chirho),
            );
        }
        PrimitiveContractChirho {
            ty_chirho,
            arity_chirho,
        }
    }
}
fn int_type_chirho(ty_chirho: IntTypeChirho) -> InterfaceBuiltinChirho {
    match ty_chirho {
        IntTypeChirho::IntChirho => InterfaceBuiltinChirho::IntChirho,
        IntTypeChirho::IntegerChirho => InterfaceBuiltinChirho::IntegerChirho,
        IntTypeChirho::BoolChirho => InterfaceBuiltinChirho::BoolChirho,
        IntTypeChirho::OrderingChirho => InterfaceBuiltinChirho::OrderingChirho,
    }
}
fn original_chirho(
    module_chirho: &str,
    occurrence_chirho: &str,
    namespace_chirho: NamespaceChirho,
) -> OriginalNameChirho {
    OriginalNameChirho {
        unit_chirho: UnitIdentityChirho::CompilerChirho,
        module_chirho: module_chirho.into(),
        namespace_chirho,
        occurrence_chirho: occurrence_chirho.into(),
    }
}
const HELPERS_CHIRHO: [(&str, IntOperationChirho); 7] = [
    ("eqInt", IntOperationChirho::EqualChirho),
    ("neInt", IntOperationChirho::NotEqualChirho),
    ("ltInt", IntOperationChirho::LessChirho),
    ("leInt", IntOperationChirho::LessEqualChirho),
    ("gtInt", IntOperationChirho::GreaterChirho),
    ("geInt", IntOperationChirho::GreaterEqualChirho),
    ("compareInt", IntOperationChirho::CompareChirho),
];
const METHODS_CHIRHO: [(&str, IntOperationChirho); 5] = [
    ("+", IntOperationChirho::AddChirho),
    ("-", IntOperationChirho::SubtractChirho),
    ("*", IntOperationChirho::MultiplyChirho),
    ("negate", IntOperationChirho::NegateChirho),
    ("fromInteger", IntOperationChirho::FromIntegerChirho),
];
pub(in crate::next_chirho) fn operation_chirho(
    name_chirho: &OriginalNameChirho,
) -> Option<ProviderChirho> {
    if let Some(operation_chirho) = integer_chirho::operation_chirho(name_chirho) {
        return Some(ProviderChirho::IntegerChirho(operation_chirho));
    }
    if name_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho
        || name_chirho.module_chirho != CLASSES_MODULE_CHIRHO
        || name_chirho.namespace_chirho != NamespaceChirho::ValueChirho
    {
        return None;
    }
    HELPERS_CHIRHO
        .iter()
        .find_map(|(text_chirho, operation_chirho)| {
            (*text_chirho == name_chirho.occurrence_chirho)
                .then_some(ProviderChirho::IntChirho(*operation_chirho))
        })
}
pub(in crate::next_chirho) fn declared_operation_chirho(
    key_chirho: &DeclaredBodyKeyChirho,
) -> Option<ProviderChirho> {
    if key_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho
        || key_chirho.module_chirho != NUM_MODULE_CHIRHO
        || key_chirho.head_chirho != WiredNameChirho::IntChirho
        || key_chirho.class_chirho
            != original_chirho(NUM_MODULE_CHIRHO, "Num", NamespaceChirho::TypeChirho)
    {
        return None;
    }
    METHODS_CHIRHO
        .iter()
        .find_map(|(name_chirho, operation_chirho)| {
            (key_chirho.method_chirho
                == original_chirho(NUM_MODULE_CHIRHO, name_chirho, NamespaceChirho::ValueChirho))
            .then_some(ProviderChirho::IntChirho(*operation_chirho))
        })
}
pub(in crate::next_chirho) fn install_policy_chirho(
    module_chirho: &str,
    policy_chirho: &mut LoweringPolicyChirho,
) {
    if module_chirho == INTEGER_MODULE_CHIRHO {
        integer_chirho::install_policy_chirho(policy_chirho);
    }
    if module_chirho == CLASSES_MODULE_CHIRHO {
        for (name_chirho, operation_chirho) in HELPERS_CHIRHO {
            policy_chirho.primitive_bindings_chirho.insert(
                original_chirho(
                    CLASSES_MODULE_CHIRHO,
                    name_chirho,
                    NamespaceChirho::ValueChirho,
                ),
                ProviderChirho::IntChirho(operation_chirho).contract_chirho(),
            );
        }
    }
    if module_chirho == NUM_MODULE_CHIRHO {
        for (name_chirho, operation_chirho) in METHODS_CHIRHO {
            policy_chirho.instance_bodies_chirho.insert(
                DeclaredBodyKeyChirho {
                    unit_chirho: UnitIdentityChirho::CompilerChirho,
                    module_chirho: NUM_MODULE_CHIRHO.into(),
                    class_chirho: original_chirho(
                        NUM_MODULE_CHIRHO,
                        "Num",
                        NamespaceChirho::TypeChirho,
                    ),
                    head_chirho: WiredNameChirho::IntChirho,
                    method_chirho: original_chirho(
                        NUM_MODULE_CHIRHO,
                        name_chirho,
                        NamespaceChirho::ValueChirho,
                    ),
                },
                ProviderChirho::IntChirho(operation_chirho).contract_chirho(),
            );
        }
    }
}

/// Shared signature lines admit a body only when every named original has a provider.
pub(in crate::next_chirho) fn validate_signature_span_chirho(
    module_chirho: &str,
    source_chirho: &str,
    start_chirho: usize,
    end_chirho: usize,
) -> Result<(), NextErrorChirho> {
    let error_chirho = || {
        NextErrorChirho::new_chirho(
            NextErrorKindChirho::UnsupportedChirho,
            "source signature has no complete declared backend contract",
        )
    };
    let selection_chirho = source_chirho
        .get(start_chirho..end_chirho)
        .ok_or_else(error_chirho)?;
    if (start_chirho > 0 && source_chirho.as_bytes()[start_chirho - 1] != b'\n')
        || !selection_chirho.ends_with('\n')
        || selection_chirho.lines().count() != 1
    {
        return Err(error_chirho());
    }
    let tokens_chirho = LexerChirho::new_chirho(selection_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
        .lex_all_chirho()
        .into_iter()
        .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
        .collect::<Vec<_>>();
    let mut cursor_chirho = 0;
    loop {
        let token_chirho = tokens_chirho
            .get(cursor_chirho)
            .filter(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::VarIdChirho)
            .ok_or_else(error_chirho)?;
        let name_chirho = token_chirho
            .span_chirho
            .text_chirho(selection_chirho)
            .ok_or_else(error_chirho)?;
        if operation_chirho(&original_chirho(
            module_chirho,
            name_chirho,
            NamespaceChirho::ValueChirho,
        ))
        .is_none()
        {
            return Err(error_chirho());
        }
        cursor_chirho += 1;
        if tokens_chirho
            .get(cursor_chirho)
            .is_some_and(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::CommaChirho)
        {
            cursor_chirho += 1;
        } else {
            break;
        }
    }
    if tokens_chirho
        .get(cursor_chirho)
        .is_none_or(|token_chirho| token_chirho.kind_chirho != RawTokenKindChirho::ColonColonChirho)
        || tokens_chirho
            .iter()
            .skip(cursor_chirho + 1)
            .any(|token_chirho| {
                !matches!(
                    token_chirho.kind_chirho,
                    RawTokenKindChirho::ConIdChirho
                        | RawTokenKindChirho::RightArrowChirho
                        | RawTokenKindChirho::EofChirho
                )
            })
    {
        return Err(error_chirho());
    }
    Ok(())
}
