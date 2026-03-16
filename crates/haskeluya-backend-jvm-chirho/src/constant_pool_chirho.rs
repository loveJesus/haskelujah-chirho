// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! JVM constant pool builder.
//!
//! The constant pool is a table of constants referenced by bytecode
//! instructions. Each entry is tagged with its type (Utf8, Class, MethodRef,
//! FieldRef, NameAndType, Integer, Long, Float, Double, String, etc.).

/// JVM constant pool entry tags.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpTagChirho {
    Utf8Chirho = 1,
    IntegerChirho = 3,
    FloatChirho = 4,
    LongChirho = 5,
    DoubleChirho = 6,
    ClassChirho = 7,
    StringChirho = 8,
    FieldRefChirho = 9,
    MethodRefChirho = 10,
    InterfaceMethodRefChirho = 11,
    NameAndTypeChirho = 12,
}

/// A single constant pool entry.
#[derive(Debug, Clone)]
pub enum CpEntryChirho {
    Utf8Chirho(String),
    IntegerChirho(i32),
    LongChirho(i64),
    FloatChirho(f32),
    DoubleChirho(f64),
    ClassChirho(u16),        // index to Utf8
    StringChirho(u16),       // index to Utf8
    FieldRefChirho(u16, u16),        // class_idx, name_and_type_idx
    MethodRefChirho(u16, u16),       // class_idx, name_and_type_idx
    InterfaceMethodRefChirho(u16, u16),
    NameAndTypeChirho(u16, u16),     // name_idx, descriptor_idx
}

/// Builder for constructing a JVM constant pool.
pub struct ConstantPoolChirho {
    entries_chirho: Vec<CpEntryChirho>,
}

impl ConstantPoolChirho {
    pub fn new_chirho() -> Self {
        Self {
            // Index 0 is unused in JVM constant pools.
            entries_chirho: vec![CpEntryChirho::Utf8Chirho(String::new())],
        }
    }

    /// Add a UTF-8 string, returning its 1-based index.
    pub fn add_utf8_chirho(&mut self, s_chirho: &str) -> u16 {
        // Check for existing entry.
        for (i_chirho, entry_chirho) in self.entries_chirho.iter().enumerate() {
            if let CpEntryChirho::Utf8Chirho(existing_chirho) = entry_chirho {
                if existing_chirho == s_chirho {
                    return i_chirho as u16;
                }
            }
        }
        self.entries_chirho
            .push(CpEntryChirho::Utf8Chirho(s_chirho.to_string()));
        (self.entries_chirho.len() - 1) as u16
    }

    /// Add a Class reference, returning its 1-based index.
    pub fn add_class_chirho(&mut self, name_chirho: &str) -> u16 {
        let name_idx_chirho = self.add_utf8_chirho(name_chirho);
        self.entries_chirho
            .push(CpEntryChirho::ClassChirho(name_idx_chirho));
        (self.entries_chirho.len() - 1) as u16
    }

    /// Add a NameAndType entry.
    pub fn add_name_and_type_chirho(&mut self, name_chirho: &str, descriptor_chirho: &str) -> u16 {
        let name_idx_chirho = self.add_utf8_chirho(name_chirho);
        let desc_idx_chirho = self.add_utf8_chirho(descriptor_chirho);
        self.entries_chirho
            .push(CpEntryChirho::NameAndTypeChirho(name_idx_chirho, desc_idx_chirho));
        (self.entries_chirho.len() - 1) as u16
    }

    /// Add a MethodRef entry.
    pub fn add_method_ref_chirho(
        &mut self,
        class_name_chirho: &str,
        method_name_chirho: &str,
        descriptor_chirho: &str,
    ) -> u16 {
        let class_idx_chirho = self.add_class_chirho(class_name_chirho);
        let nat_idx_chirho = self.add_name_and_type_chirho(method_name_chirho, descriptor_chirho);
        self.entries_chirho
            .push(CpEntryChirho::MethodRefChirho(class_idx_chirho, nat_idx_chirho));
        (self.entries_chirho.len() - 1) as u16
    }

    /// Add an Integer constant.
    pub fn add_integer_chirho(&mut self, val_chirho: i32) -> u16 {
        self.entries_chirho
            .push(CpEntryChirho::IntegerChirho(val_chirho));
        (self.entries_chirho.len() - 1) as u16
    }

    /// Add a Long constant (occupies two pool slots).
    pub fn add_long_chirho(&mut self, val_chirho: i64) -> u16 {
        self.entries_chirho
            .push(CpEntryChirho::LongChirho(val_chirho));
        let idx_chirho = (self.entries_chirho.len() - 1) as u16;
        // Long/Double entries take two slots.
        self.entries_chirho
            .push(CpEntryChirho::Utf8Chirho(String::new()));
        idx_chirho
    }

    /// Serialize the constant pool to bytes.
    pub fn to_bytes_chirho(&self) -> Vec<u8> {
        let mut bytes_chirho = Vec::new();
        // constant_pool_count = entries.len() (includes unused slot 0)
        let count_chirho = self.entries_chirho.len() as u16;
        bytes_chirho.extend_from_slice(&count_chirho.to_be_bytes());

        // Skip index 0 (unused).
        for entry_chirho in &self.entries_chirho[1..] {
            match entry_chirho {
                CpEntryChirho::Utf8Chirho(s_chirho) => {
                    bytes_chirho.push(CpTagChirho::Utf8Chirho as u8);
                    let len_chirho = s_chirho.len() as u16;
                    bytes_chirho.extend_from_slice(&len_chirho.to_be_bytes());
                    bytes_chirho.extend_from_slice(s_chirho.as_bytes());
                }
                CpEntryChirho::IntegerChirho(v_chirho) => {
                    bytes_chirho.push(CpTagChirho::IntegerChirho as u8);
                    bytes_chirho.extend_from_slice(&v_chirho.to_be_bytes());
                }
                CpEntryChirho::LongChirho(v_chirho) => {
                    bytes_chirho.push(CpTagChirho::LongChirho as u8);
                    bytes_chirho.extend_from_slice(&v_chirho.to_be_bytes());
                }
                CpEntryChirho::FloatChirho(v_chirho) => {
                    bytes_chirho.push(CpTagChirho::FloatChirho as u8);
                    bytes_chirho.extend_from_slice(&v_chirho.to_be_bytes());
                }
                CpEntryChirho::DoubleChirho(v_chirho) => {
                    bytes_chirho.push(CpTagChirho::DoubleChirho as u8);
                    bytes_chirho.extend_from_slice(&v_chirho.to_be_bytes());
                }
                CpEntryChirho::ClassChirho(idx_chirho) => {
                    bytes_chirho.push(CpTagChirho::ClassChirho as u8);
                    bytes_chirho.extend_from_slice(&idx_chirho.to_be_bytes());
                }
                CpEntryChirho::StringChirho(idx_chirho) => {
                    bytes_chirho.push(CpTagChirho::StringChirho as u8);
                    bytes_chirho.extend_from_slice(&idx_chirho.to_be_bytes());
                }
                CpEntryChirho::FieldRefChirho(c_chirho, nat_chirho) => {
                    bytes_chirho.push(CpTagChirho::FieldRefChirho as u8);
                    bytes_chirho.extend_from_slice(&c_chirho.to_be_bytes());
                    bytes_chirho.extend_from_slice(&nat_chirho.to_be_bytes());
                }
                CpEntryChirho::MethodRefChirho(c_chirho, nat_chirho) => {
                    bytes_chirho.push(CpTagChirho::MethodRefChirho as u8);
                    bytes_chirho.extend_from_slice(&c_chirho.to_be_bytes());
                    bytes_chirho.extend_from_slice(&nat_chirho.to_be_bytes());
                }
                CpEntryChirho::InterfaceMethodRefChirho(c_chirho, nat_chirho) => {
                    bytes_chirho.push(CpTagChirho::InterfaceMethodRefChirho as u8);
                    bytes_chirho.extend_from_slice(&c_chirho.to_be_bytes());
                    bytes_chirho.extend_from_slice(&nat_chirho.to_be_bytes());
                }
                CpEntryChirho::NameAndTypeChirho(n_chirho, d_chirho) => {
                    bytes_chirho.push(CpTagChirho::NameAndTypeChirho as u8);
                    bytes_chirho.extend_from_slice(&n_chirho.to_be_bytes());
                    bytes_chirho.extend_from_slice(&d_chirho.to_be_bytes());
                }
            }
        }
        bytes_chirho
    }

    /// Number of entries (including unused slot 0).
    pub fn len_chirho(&self) -> usize {
        self.entries_chirho.len()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn constant_pool_utf8_chirho() {
        let mut cp_chirho = ConstantPoolChirho::new_chirho();
        let idx1_chirho = cp_chirho.add_utf8_chirho("hello");
        let idx2_chirho = cp_chirho.add_utf8_chirho("hello"); // dedup
        assert_eq!(idx1_chirho, idx2_chirho);
        assert_eq!(cp_chirho.len_chirho(), 2); // slot 0 + "hello"
    }

    #[test]
    fn constant_pool_class_chirho() {
        let mut cp_chirho = ConstantPoolChirho::new_chirho();
        let idx_chirho = cp_chirho.add_class_chirho("java/lang/Object");
        assert!(idx_chirho > 0);
    }

    #[test]
    fn constant_pool_method_ref_chirho() {
        let mut cp_chirho = ConstantPoolChirho::new_chirho();
        let idx_chirho = cp_chirho.add_method_ref_chirho(
            "java/lang/Object",
            "<init>",
            "()V",
        );
        assert!(idx_chirho > 0);
    }

    #[test]
    fn constant_pool_serialization_chirho() {
        let mut cp_chirho = ConstantPoolChirho::new_chirho();
        cp_chirho.add_utf8_chirho("test");
        let bytes_chirho = cp_chirho.to_bytes_chirho();
        // First 2 bytes = count (2 entries: slot 0 + "test")
        assert_eq!(bytes_chirho[0], 0);
        assert_eq!(bytes_chirho[1], 2);
        // Tag 1 = Utf8
        assert_eq!(bytes_chirho[2], 1);
    }
}
