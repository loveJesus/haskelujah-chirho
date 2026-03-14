// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Erlang External Term Format (ETF) serialization.
//!
//! BEAM files use ETF for encoding literal tables, atom tables, and
//! other chunk data. This module provides serialization for the subset
//! of ETF types needed by the compiler.

/// ETF version byte (always 131 for modern Erlang).
pub const ETF_VERSION_CHIRHO: u8 = 131;

/// ETF type tags.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtfTagChirho {
    SmallIntegerChirho = 97,
    IntegerChirho = 98,
    AtomChirho = 100,
    SmallTupleChirho = 104,
    LargeTupleChirho = 105,
    NilChirho = 106,
    StringChirho = 107,
    ListChirho = 108,
    BinaryChirho = 109,
    SmallAtomUtf8Chirho = 119,
    AtomUtf8Chirho = 118,
}

/// Serialize an atom (UTF-8 string) to ETF bytes.
pub fn encode_atom_chirho(atom_chirho: &str) -> Vec<u8> {
    let mut bytes_chirho = Vec::new();
    let len_chirho = atom_chirho.len();
    if len_chirho <= 255 {
        bytes_chirho.push(EtfTagChirho::SmallAtomUtf8Chirho as u8);
        bytes_chirho.push(len_chirho as u8);
    } else {
        bytes_chirho.push(EtfTagChirho::AtomUtf8Chirho as u8);
        bytes_chirho.extend_from_slice(&(len_chirho as u16).to_be_bytes());
    }
    bytes_chirho.extend_from_slice(atom_chirho.as_bytes());
    bytes_chirho
}

/// Serialize a small integer (0–255) to ETF bytes.
pub fn encode_small_int_chirho(val_chirho: u8) -> Vec<u8> {
    vec![EtfTagChirho::SmallIntegerChirho as u8, val_chirho]
}

/// Serialize a 32-bit integer to ETF bytes.
pub fn encode_integer_chirho(val_chirho: i32) -> Vec<u8> {
    if val_chirho >= 0 && val_chirho <= 255 {
        return encode_small_int_chirho(val_chirho as u8);
    }
    let mut bytes_chirho = vec![EtfTagChirho::IntegerChirho as u8];
    bytes_chirho.extend_from_slice(&val_chirho.to_be_bytes());
    bytes_chirho
}

/// Serialize a tuple to ETF bytes.
pub fn encode_tuple_chirho(elements_chirho: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes_chirho = Vec::new();
    let arity_chirho = elements_chirho.len();
    if arity_chirho <= 255 {
        bytes_chirho.push(EtfTagChirho::SmallTupleChirho as u8);
        bytes_chirho.push(arity_chirho as u8);
    } else {
        bytes_chirho.push(EtfTagChirho::LargeTupleChirho as u8);
        bytes_chirho.extend_from_slice(&(arity_chirho as u32).to_be_bytes());
    }
    for elem_chirho in elements_chirho {
        bytes_chirho.extend_from_slice(elem_chirho);
    }
    bytes_chirho
}

/// Serialize an empty list (nil) to ETF bytes.
pub fn encode_nil_chirho() -> Vec<u8> {
    vec![EtfTagChirho::NilChirho as u8]
}

/// Serialize a list to ETF bytes.
pub fn encode_list_chirho(elements_chirho: &[Vec<u8>]) -> Vec<u8> {
    if elements_chirho.is_empty() {
        return encode_nil_chirho();
    }
    let mut bytes_chirho = vec![EtfTagChirho::ListChirho as u8];
    bytes_chirho.extend_from_slice(&(elements_chirho.len() as u32).to_be_bytes());
    for elem_chirho in elements_chirho {
        bytes_chirho.extend_from_slice(elem_chirho);
    }
    // Proper list tail = nil
    bytes_chirho.push(EtfTagChirho::NilChirho as u8);
    bytes_chirho
}

/// Serialize a binary (byte string) to ETF bytes.
pub fn encode_binary_chirho(data_chirho: &[u8]) -> Vec<u8> {
    let mut bytes_chirho = vec![EtfTagChirho::BinaryChirho as u8];
    bytes_chirho.extend_from_slice(&(data_chirho.len() as u32).to_be_bytes());
    bytes_chirho.extend_from_slice(data_chirho);
    bytes_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn encode_small_atom_chirho() {
        let bytes_chirho = encode_atom_chirho("ok");
        assert_eq!(bytes_chirho[0], EtfTagChirho::SmallAtomUtf8Chirho as u8);
        assert_eq!(bytes_chirho[1], 2); // length
        assert_eq!(&bytes_chirho[2..], b"ok");
    }

    #[test]
    fn encode_small_integer_chirho() {
        let bytes_chirho = encode_small_int_chirho(42);
        assert_eq!(bytes_chirho, vec![97, 42]);
    }

    #[test]
    fn encode_integer_negative_chirho() {
        let bytes_chirho = encode_integer_chirho(-1);
        assert_eq!(bytes_chirho[0], EtfTagChirho::IntegerChirho as u8);
    }

    #[test]
    fn encode_tuple_pair_chirho() {
        let elems_chirho = vec![
            encode_atom_chirho("ok"),
            encode_small_int_chirho(42),
        ];
        let bytes_chirho = encode_tuple_chirho(&elems_chirho);
        assert_eq!(bytes_chirho[0], EtfTagChirho::SmallTupleChirho as u8);
        assert_eq!(bytes_chirho[1], 2); // arity
    }

    #[test]
    fn encode_empty_list_chirho() {
        let bytes_chirho = encode_list_chirho(&[]);
        assert_eq!(bytes_chirho, vec![EtfTagChirho::NilChirho as u8]);
    }

    #[test]
    fn encode_nonempty_list_chirho() {
        let elems_chirho = vec![encode_small_int_chirho(1), encode_small_int_chirho(2)];
        let bytes_chirho = encode_list_chirho(&elems_chirho);
        assert_eq!(bytes_chirho[0], EtfTagChirho::ListChirho as u8);
        // Last byte should be nil (proper list tail)
        assert_eq!(*bytes_chirho.last().unwrap(), EtfTagChirho::NilChirho as u8);
    }

    #[test]
    fn encode_binary_hello_chirho() {
        let bytes_chirho = encode_binary_chirho(b"hello");
        assert_eq!(bytes_chirho[0], EtfTagChirho::BinaryChirho as u8);
        assert_eq!(&bytes_chirho[5..], b"hello");
    }
}
