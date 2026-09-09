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
