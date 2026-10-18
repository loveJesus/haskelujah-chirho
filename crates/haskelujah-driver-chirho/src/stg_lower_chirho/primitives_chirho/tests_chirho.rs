// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{ArgSourceChirho, LowerCtxChirho, ValueChirho};
use haskelujah_runtime_chirho::MachineChirho;

fn execute_chirho(name_chirho: &str) -> Result<ValueChirho, String> {
    let mut context_chirho = LowerCtxChirho::new_chirho();
    let entry_chirho = context_chirho.emit_named_primitive_chirho(
        name_chirho,
        [17, 25]
            .into_iter()
            .map(|value_chirho| ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(value_chirho)))
            .collect(),
    );
    MachineChirho::new_chirho(context_chirho.code_chirho)
        .run_chirho(entry_chirho)
        .map_err(|error_chirho| error_chirho.to_string())
}

#[test]
fn unknown_primitive_cannot_masquerade_as_addition_chirho() {
    let error_chirho = execute_chirho("noSuchPrimitiveChirho#")
        .expect_err("an unknown primitive must not compute 17 + 25");
    assert!(error_chirho.contains("unsupported STG primitive: noSuchPrimitiveChirho#"));
}

#[test]
fn supported_addition_still_executes_chirho() {
    assert_eq!(execute_chirho("+#").unwrap(), ValueChirho::IntChirho(42));
}

/// The checked `toEnum` rows fail through `error` with a computed message:
/// `"Enum.toEnum{Word8}: tag (" ++# (showInt# tag ++# ") is outside ...")`.
/// The `error` route carries that text exactly, with no `error#` mapping.
/// workflow: language-features-chirho/integral-enum-rows-chirho
#[test]
fn a_computed_error_message_fails_with_its_exact_text_chirho() {
    let text_chirho = |text_chirho: &str| {
        ArgSourceChirho::StaticChirho(ValueChirho::StringChirho(text_chirho.to_string()))
    };
    let thunk_chirho = |code_ptr_chirho: u32| ArgSourceChirho::ThunkCodeChirho {
        code_ptr_chirho,
        captures_chirho: Vec::new(),
    };
    let mut context_chirho = LowerCtxChirho::new_chirho();
    let shown_tag_chirho = context_chirho.emit_named_primitive_chirho(
        "showInt#",
        vec![ArgSourceChirho::StaticChirho(ValueChirho::IntChirho(300))],
    );
    let suffix_chirho = context_chirho.emit_named_primitive_chirho(
        "++#",
        vec![
            thunk_chirho(shown_tag_chirho),
            text_chirho(") is outside of enumeration's range (0,255)"),
        ],
    );
    let message_chirho = context_chirho.emit_named_primitive_chirho(
        "++#",
        vec![
            text_chirho("Enum.toEnum{Word8}: tag ("),
            thunk_chirho(suffix_chirho),
        ],
    );
    let entry_chirho =
        context_chirho.emit_named_primitive_chirho("error", vec![thunk_chirho(message_chirho)]);
    let error_chirho = MachineChirho::new_chirho(context_chirho.code_chirho)
        .run_chirho(entry_chirho)
        .expect_err("a checked toEnum failure must not return a value")
        .to_string();
    assert!(
        error_chirho
            .contains("Enum.toEnum{Word8}: tag (300) is outside of enumeration's range (0,255)"),
        "{error_chirho}"
    );
}
