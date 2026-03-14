// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! BEAM module file generation from Core IR.
//!
//! Generates .beam files following the IFF (Interchange File Format)
//! structure used by BEAM. A .beam file consists of:
//!
//! ```text
//! "FOR1" <size:u32> "BEAM"
//!   <chunk_id:4 bytes> <chunk_size:u32> <chunk_data> [padding]
//!   ...
//! ```

use rhasky_core_chirho::expr_chirho::CoreModuleChirho;

use crate::{BeamConfigChirho, BeamOutputChirho};

/// BEAM file magic: "FOR1"
const BEAM_MAGIC_CHIRHO: &[u8; 4] = b"FOR1";
/// BEAM format tag
const BEAM_TAG_CHIRHO: &[u8; 4] = b"BEAM";

/// Compile a Core module to a BEAM file.
pub fn compile_core_to_beam_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &BeamConfigChirho,
) -> Result<BeamOutputChirho, String> {
    let mut chunks_chirho: Vec<Vec<u8>> = Vec::new();

    // AtU8 chunk: atom table
    let atom_table_chirho = build_atom_table_chirho(module_chirho, config_chirho);
    chunks_chirho.push(build_chunk_chirho(b"AtU8", &atom_table_chirho));

    // Code chunk: instruction bytecode (stub for now)
    let code_chirho = build_code_chunk_chirho(module_chirho, config_chirho);
    chunks_chirho.push(build_chunk_chirho(b"Code", &code_chirho));

    // StrT chunk: string table (empty for now)
    chunks_chirho.push(build_chunk_chirho(b"StrT", &[]));

    // ImpT chunk: import table (empty for now)
    chunks_chirho.push(build_chunk_chirho(b"ImpT", &0u32.to_be_bytes()));

    // ExpT chunk: export table (empty for now)
    chunks_chirho.push(build_chunk_chirho(b"ExpT", &0u32.to_be_bytes()));

    // Combine all chunks.
    let mut body_chirho = Vec::new();
    body_chirho.extend_from_slice(BEAM_TAG_CHIRHO);
    for chunk_chirho in &chunks_chirho {
        body_chirho.extend_from_slice(chunk_chirho);
    }

    // Build the full BEAM file.
    let mut beam_bytes_chirho = Vec::new();
    beam_bytes_chirho.extend_from_slice(BEAM_MAGIC_CHIRHO);
    beam_bytes_chirho.extend_from_slice(&(body_chirho.len() as u32).to_be_bytes());
    beam_bytes_chirho.extend_from_slice(&body_chirho);

    Ok(BeamOutputChirho {
        module_name_chirho: config_chirho.module_name_chirho.clone(),
        beam_bytes_chirho,
    })
}

/// Build a single IFF chunk: 4-byte ID + size + data + padding.
fn build_chunk_chirho(id_chirho: &[u8; 4], data_chirho: &[u8]) -> Vec<u8> {
    let mut chunk_chirho = Vec::new();
    chunk_chirho.extend_from_slice(id_chirho);
    chunk_chirho.extend_from_slice(&(data_chirho.len() as u32).to_be_bytes());
    chunk_chirho.extend_from_slice(data_chirho);
    // Pad to 4-byte boundary.
    while chunk_chirho.len() % 4 != 0 {
        chunk_chirho.push(0);
    }
    chunk_chirho
}

/// Build the atom table chunk data.
fn build_atom_table_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &BeamConfigChirho,
) -> Vec<u8> {
    let mut atoms_chirho: Vec<String> = Vec::new();

    // First atom must be the module name.
    atoms_chirho.push(config_chirho.module_name_chirho.clone());

    // Add binding names as atoms.
    for binding_chirho in &module_chirho.bindings_chirho {
        let name_chirho = &binding_chirho.binder_chirho.name_chirho;
        if !atoms_chirho.contains(name_chirho) {
            atoms_chirho.push(name_chirho.clone());
        }
    }

    // Standard atoms used by the runtime.
    for std_atom_chirho in &["true", "false", "undefined", "ok", "error"] {
        let s_chirho = std_atom_chirho.to_string();
        if !atoms_chirho.contains(&s_chirho) {
            atoms_chirho.push(s_chirho);
        }
    }

    // Serialize.
    let mut data_chirho = Vec::new();
    data_chirho.extend_from_slice(&(atoms_chirho.len() as u32).to_be_bytes());
    for atom_chirho in &atoms_chirho {
        let bytes_chirho = atom_chirho.as_bytes();
        data_chirho.push(bytes_chirho.len() as u8);
        data_chirho.extend_from_slice(bytes_chirho);
    }
    data_chirho
}

/// Build the code chunk data (stub: header only, no instructions).
fn build_code_chunk_chirho(
    _module_chirho: &CoreModuleChirho,
    config_chirho: &BeamConfigChirho,
) -> Vec<u8> {
    let mut data_chirho = Vec::new();

    // Code chunk header:
    //   sub_size: u32 (header size = 16)
    //   instruction_set: u32
    //   opcode_max: u32
    //   label_count: u32
    //   function_count: u32
    data_chirho.extend_from_slice(&16u32.to_be_bytes()); // sub_size
    data_chirho.extend_from_slice(&config_chirho.instruction_set_chirho.to_be_bytes());
    data_chirho.extend_from_slice(&config_chirho.max_opcode_chirho.to_be_bytes());
    data_chirho.extend_from_slice(&0u32.to_be_bytes()); // label_count
    data_chirho.extend_from_slice(&0u32.to_be_bytes()); // function_count

    // TODO: emit actual BEAM instructions here.

    data_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_core_chirho::expr_chirho::{
        BinderChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho, CoreLitChirho,
        CoreModuleChirho,
    };
    use rhasky_span_chirho::SpanChirho;
    use rhasky_typing_chirho::ty_chirho::TyChirho;

    #[test]
    fn compile_empty_beam_module_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![],
            names_chirho: Default::default(),
        };
        let config_chirho = BeamConfigChirho::default();
        let result_chirho = compile_core_to_beam_chirho(&module_chirho, &config_chirho);
        assert!(result_chirho.is_ok());
        let output_chirho = result_chirho.unwrap();
        // Check magic: FOR1
        assert_eq!(&output_chirho.beam_bytes_chirho[0..4], b"FOR1");
        // Check BEAM tag
        assert_eq!(&output_chirho.beam_bytes_chirho[8..12], b"BEAM");
    }

    #[test]
    fn beam_module_name_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![],
            names_chirho: Default::default(),
        };
        let mut config_chirho = BeamConfigChirho::default();
        config_chirho.module_name_chirho = "my_module".to_string();
        let output_chirho = compile_core_to_beam_chirho(&module_chirho, &config_chirho).unwrap();
        assert_eq!(output_chirho.module_name_chirho, "my_module");
    }

    #[test]
    fn beam_atom_table_includes_bindings_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: CoreIdChirho(0),
                    name_chirho: "main".to_string(),
                    ty_chirho: TyChirho::int_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(42)),
                is_rec_chirho: false,
            }],
            names_chirho: Default::default(),
        };
        let config_chirho = BeamConfigChirho::default();
        let atom_data_chirho = build_atom_table_chirho(&module_chirho, &config_chirho);
        // First 4 bytes = atom count (at least module name + main + standard atoms)
        let count_chirho = u32::from_be_bytes([
            atom_data_chirho[0],
            atom_data_chirho[1],
            atom_data_chirho[2],
            atom_data_chirho[3],
        ]);
        assert!(count_chirho >= 6); // module name + main + true + false + undefined + ok + error
    }

    #[test]
    fn beam_chunk_padding_chirho() {
        let chunk_chirho = build_chunk_chirho(b"Test", &[1, 2, 3]);
        // 4 (id) + 4 (size) + 3 (data) + 1 (padding) = 12 bytes
        assert_eq!(chunk_chirho.len() % 4, 0);
    }
}
