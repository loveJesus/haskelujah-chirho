// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! JVM class file generation from Core IR.
//!
//! Generates .class files following the JVM specification. Each Haskell
//! module produces one main class, with data constructors as inner classes
//! and closures as anonymous inner classes.

use rhasky_core_chirho::expr_chirho::CoreModuleChirho;

use crate::constant_pool_chirho::ConstantPoolChirho;
use crate::{ClassFileChirho, JvmConfigChirho, JvmOutputChirho};

/// JVM class file magic number.
const MAGIC_CHIRHO: u32 = 0xCAFEBABE;

/// JVM access flags.
const ACC_PUBLIC_CHIRHO: u16 = 0x0001;
const ACC_SUPER_CHIRHO: u16 = 0x0020;
#[allow(dead_code)]
const ACC_STATIC_CHIRHO: u16 = 0x0008;

/// Compile a Core module to JVM class files.
pub fn compile_core_to_class_chirho(
    module_chirho: &CoreModuleChirho,
    config_chirho: &JvmConfigChirho,
) -> Result<JvmOutputChirho, String> {
    let class_name_chirho = format!("{}/Main", config_chirho.package_chirho);

    let mut cp_chirho = ConstantPoolChirho::new_chirho();

    // Add required constant pool entries.
    let this_class_idx_chirho = cp_chirho.add_class_chirho(&class_name_chirho);
    let super_class_idx_chirho = cp_chirho.add_class_chirho("java/lang/Object");

    // Add <init> method ref for java/lang/Object.
    let _init_ref_chirho =
        cp_chirho.add_method_ref_chirho("java/lang/Object", "<init>", "()V");

    // Add Code attribute name.
    let _code_attr_chirho = cp_chirho.add_utf8_chirho("Code");

    // Build class file bytes.
    let mut bytes_chirho: Vec<u8> = Vec::new();

    // Magic number
    bytes_chirho.extend_from_slice(&MAGIC_CHIRHO.to_be_bytes());

    // Minor version
    bytes_chirho.extend_from_slice(&0u16.to_be_bytes());

    // Major version
    bytes_chirho.extend_from_slice(&config_chirho.major_version_chirho.to_be_bytes());

    // Constant pool
    bytes_chirho.extend_from_slice(&cp_chirho.to_bytes_chirho());

    // Access flags: public + super
    let access_flags_chirho = ACC_PUBLIC_CHIRHO | ACC_SUPER_CHIRHO;
    bytes_chirho.extend_from_slice(&access_flags_chirho.to_be_bytes());

    // This class
    bytes_chirho.extend_from_slice(&this_class_idx_chirho.to_be_bytes());

    // Super class
    bytes_chirho.extend_from_slice(&super_class_idx_chirho.to_be_bytes());

    // Interfaces count = 0
    bytes_chirho.extend_from_slice(&0u16.to_be_bytes());

    // Fields count = 0 (for now)
    bytes_chirho.extend_from_slice(&0u16.to_be_bytes());

    // Methods count = 0 (for now — TODO: add main and binding methods)
    let _binding_count_chirho = module_chirho.bindings_chirho.len();
    bytes_chirho.extend_from_slice(&0u16.to_be_bytes());

    // Attributes count = 0
    bytes_chirho.extend_from_slice(&0u16.to_be_bytes());

    Ok(JvmOutputChirho {
        classes_chirho: vec![ClassFileChirho {
            name_chirho: class_name_chirho,
            bytes_chirho,
        }],
    })
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhasky_core_chirho::expr_chirho::CoreModuleChirho;

    #[test]
    fn compile_empty_module_jvm_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        let config_chirho = JvmConfigChirho::default();
        let result_chirho = compile_core_to_class_chirho(&module_chirho, &config_chirho);
        assert!(result_chirho.is_ok());
        let output_chirho = result_chirho.unwrap();
        assert_eq!(output_chirho.classes_chirho.len(), 1);
        let class_chirho = &output_chirho.classes_chirho[0];
        // Check magic number.
        assert_eq!(&class_chirho.bytes_chirho[0..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
    }

    #[test]
    fn class_name_from_config_chirho() {
        let module_chirho = CoreModuleChirho {
            name_chirho: "Test".to_string(),
            bindings_chirho: vec![],
            names_chirho: Default::default(),
            specialize_pragmas_chirho: Default::default(),
            foreign_exports_chirho: vec![],
        };
        let mut config_chirho = JvmConfigChirho::default();
        config_chirho.package_chirho = "com/example".to_string();
        let output_chirho = compile_core_to_class_chirho(&module_chirho, &config_chirho).unwrap();
        assert_eq!(output_chirho.classes_chirho[0].name_chirho, "com/example/Main");
    }
}
