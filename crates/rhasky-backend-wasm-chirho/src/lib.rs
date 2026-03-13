// For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life.

pub fn compile_to_wasm_stub_chirho(module_name_chirho: &str) -> Vec<u8> {
    format!("rhasky-wasm-stub:{module_name_chirho}").into_bytes()
}

