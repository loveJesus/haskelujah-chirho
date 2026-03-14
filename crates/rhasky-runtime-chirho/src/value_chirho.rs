// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Runtime values and closure representation
//!
//! Defines the core runtime types for the STG machine:
//! - `ValueChirho` — the universal runtime value type (tagged union)
//! - `ClosureChirho` — a heap-allocated closure (code pointer + free vars)
//! - `InfoTagChirho` — tag distinguishing closure types (function, thunk,
//!   constructor, indirection, blackhole)
//! - `DataConTagChirho` — tag for algebraic data constructors

use std::fmt;

/// Unique identifier for a code entry point (function / thunk body).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodePtrChirho(pub u32);

/// Tag distinguishing the kind of closure on the heap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfoTagChirho {
    /// A function closure: has arity, waits for arguments.
    FunChirho,
    /// A thunk: a suspended computation, evaluated at most once.
    ThunkChirho,
    /// A data constructor applied to all its fields.
    ConChirho,
    /// An indirection: points to the result after thunk evaluation.
    IndChirho,
    /// A blackhole: a thunk currently being evaluated (cycle detection).
    BlackholeChirho,
    /// A partial application: a function applied to fewer args than its arity.
    PapChirho,
}

impl fmt::Display for InfoTagChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FunChirho => write!(f_chirho, "FUN"),
            Self::ThunkChirho => write!(f_chirho, "THUNK"),
            Self::ConChirho => write!(f_chirho, "CON"),
            Self::IndChirho => write!(f_chirho, "IND"),
            Self::BlackholeChirho => write!(f_chirho, "BLACKHOLE"),
            Self::PapChirho => write!(f_chirho, "PAP"),
        }
    }
}

/// Constructor tag for algebraic data types.
///
/// E.g. `True` = tag 0, `False` = tag 1 within the `Bool` type;
/// `Nothing` = tag 0, `Just` = tag 1 within `Maybe`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DataConTagChirho(pub u16);

/// The info table for a heap closure. Every closure on the heap starts
/// with a pointer to its info table (or, in our model, stores it inline).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoTableChirho {
    /// What kind of closure this is.
    pub tag_chirho: InfoTagChirho,
    /// For FUN: the arity (number of args before entering the body).
    /// For CON: the number of fields.
    /// For THUNK/IND/BLACKHOLE: 0.
    /// For PAP: the remaining arity (original arity - applied args).
    pub arity_chirho: u16,
    /// For CON: the constructor tag.
    pub con_tag_chirho: DataConTagChirho,
    /// The code entry point (index into the code table).
    /// For THUNK: the code that computes the value.
    /// For FUN: the function body.
    /// For CON/IND/BLACKHOLE: unused (set to 0).
    pub entry_chirho: CodePtrChirho,
    /// A human-readable name for debugging.
    pub name_chirho: String,
}

impl InfoTableChirho {
    /// Create an info table for a function closure.
    pub fn fun_chirho(arity_chirho: u16, entry_chirho: CodePtrChirho, name_chirho: &str) -> Self {
        Self {
            tag_chirho: InfoTagChirho::FunChirho,
            arity_chirho,
            con_tag_chirho: DataConTagChirho(0),
            entry_chirho,
            name_chirho: name_chirho.to_string(),
        }
    }

    /// Create an info table for a thunk.
    pub fn thunk_chirho(entry_chirho: CodePtrChirho, name_chirho: &str) -> Self {
        Self {
            tag_chirho: InfoTagChirho::ThunkChirho,
            arity_chirho: 0,
            con_tag_chirho: DataConTagChirho(0),
            entry_chirho,
            name_chirho: name_chirho.to_string(),
        }
    }

    /// Create an info table for a data constructor.
    pub fn con_chirho(con_tag_chirho: DataConTagChirho, arity_chirho: u16, name_chirho: &str) -> Self {
        Self {
            tag_chirho: InfoTagChirho::ConChirho,
            arity_chirho,
            con_tag_chirho,
            entry_chirho: CodePtrChirho(0),
            name_chirho: name_chirho.to_string(),
        }
    }

    /// Create an info table for a partial application.
    pub fn pap_chirho(remaining_arity_chirho: u16) -> Self {
        Self {
            tag_chirho: InfoTagChirho::PapChirho,
            arity_chirho: remaining_arity_chirho,
            con_tag_chirho: DataConTagChirho(0),
            entry_chirho: CodePtrChirho(0),
            name_chirho: "$PAP".to_string(),
        }
    }

    /// Create an indirection info table.
    pub fn ind_chirho() -> Self {
        Self {
            tag_chirho: InfoTagChirho::IndChirho,
            arity_chirho: 0,
            con_tag_chirho: DataConTagChirho(0),
            entry_chirho: CodePtrChirho(0),
            name_chirho: "$IND".to_string(),
        }
    }

    /// Create a blackhole info table.
    pub fn blackhole_chirho() -> Self {
        Self {
            tag_chirho: InfoTagChirho::BlackholeChirho,
            arity_chirho: 0,
            con_tag_chirho: DataConTagChirho(0),
            entry_chirho: CodePtrChirho(0),
            name_chirho: "$BLACKHOLE".to_string(),
        }
    }
}

/// A runtime value — the universal type that flows through the STG machine.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueChirho {
    /// A pointer to a heap-allocated closure.
    HeapPtrChirho(HeapAddrChirho),
    /// An unboxed integer (Int#).
    IntChirho(i64),
    /// An unboxed float (Double#).
    FloatChirho(f64),
    /// An unboxed character (Char#).
    CharChirho(char),
    /// An unboxed boolean (for internal use).
    BoolChirho(bool),
    /// An unboxed string (Addr# / String).
    StringChirho(String),
}

impl fmt::Display for ValueChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HeapPtrChirho(addr_chirho) => write!(f_chirho, "@{}", addr_chirho.0),
            Self::IntChirho(v_chirho) => write!(f_chirho, "{}#", v_chirho),
            Self::FloatChirho(v_chirho) => write!(f_chirho, "{}##", v_chirho),
            Self::CharChirho(v_chirho) => write!(f_chirho, "'{}#'", v_chirho),
            Self::BoolChirho(v_chirho) => write!(f_chirho, "{}#", v_chirho),
            Self::StringChirho(v_chirho) => write!(f_chirho, "\"{}\"#", v_chirho),
        }
    }
}

/// Address of a closure on the heap (index into the heap array).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeapAddrChirho(pub u32);

/// A closure on the heap: info table + payload (free variables / fields).
#[derive(Debug, Clone, PartialEq)]
pub struct ClosureChirho {
    pub info_chirho: InfoTableChirho,
    /// The payload: free variables (for FUN/THUNK), constructor fields
    /// (for CON), partially applied arguments (for PAP), or target pointer
    /// (for IND — stored as a single HeapPtr element).
    pub payload_chirho: Vec<ValueChirho>,
}

impl ClosureChirho {
    /// Create a function closure.
    pub fn fun_chirho(
        arity_chirho: u16,
        entry_chirho: CodePtrChirho,
        name_chirho: &str,
        free_vars_chirho: Vec<ValueChirho>,
    ) -> Self {
        Self {
            info_chirho: InfoTableChirho::fun_chirho(arity_chirho, entry_chirho, name_chirho),
            payload_chirho: free_vars_chirho,
        }
    }

    /// Create a thunk closure.
    pub fn thunk_chirho(
        entry_chirho: CodePtrChirho,
        name_chirho: &str,
        free_vars_chirho: Vec<ValueChirho>,
    ) -> Self {
        Self {
            info_chirho: InfoTableChirho::thunk_chirho(entry_chirho, name_chirho),
            payload_chirho: free_vars_chirho,
        }
    }

    /// Create a saturated data constructor closure.
    pub fn con_chirho(
        con_tag_chirho: DataConTagChirho,
        name_chirho: &str,
        fields_chirho: Vec<ValueChirho>,
    ) -> Self {
        let arity_chirho = fields_chirho.len() as u16;
        Self {
            info_chirho: InfoTableChirho::con_chirho(con_tag_chirho, arity_chirho, name_chirho),
            payload_chirho: fields_chirho,
        }
    }

    /// Create an indirection to another closure.
    pub fn ind_chirho(target_chirho: HeapAddrChirho) -> Self {
        Self {
            info_chirho: InfoTableChirho::ind_chirho(),
            payload_chirho: vec![ValueChirho::HeapPtrChirho(target_chirho)],
        }
    }

    /// Create a blackhole (marks a thunk being evaluated).
    pub fn blackhole_chirho() -> Self {
        Self {
            info_chirho: InfoTableChirho::blackhole_chirho(),
            payload_chirho: vec![],
        }
    }

    /// Create a tombstone (dead closure marker for the GC).
    pub fn tombstone_chirho() -> Self {
        Self {
            info_chirho: InfoTableChirho {
                tag_chirho: InfoTagChirho::BlackholeChirho,
                arity_chirho: 0,
                con_tag_chirho: DataConTagChirho(0),
                entry_chirho: CodePtrChirho(0),
                name_chirho: "$DEAD".to_string(),
            },
            payload_chirho: vec![],
        }
    }

    /// Create a partial application.
    pub fn pap_chirho(
        remaining_arity_chirho: u16,
        fun_addr_chirho: HeapAddrChirho,
        applied_args_chirho: Vec<ValueChirho>,
    ) -> Self {
        let mut payload_chirho = vec![ValueChirho::HeapPtrChirho(fun_addr_chirho)];
        payload_chirho.extend(applied_args_chirho);
        Self {
            info_chirho: InfoTableChirho::pap_chirho(remaining_arity_chirho),
            payload_chirho,
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn create_fun_closure_chirho() {
        let closure_chirho = ClosureChirho::fun_chirho(
            2,
            CodePtrChirho(0),
            "add",
            vec![],
        );
        assert_eq!(closure_chirho.info_chirho.tag_chirho, InfoTagChirho::FunChirho);
        assert_eq!(closure_chirho.info_chirho.arity_chirho, 2);
        assert!(closure_chirho.payload_chirho.is_empty());
    }

    #[test]
    fn create_thunk_closure_chirho() {
        let closure_chirho = ClosureChirho::thunk_chirho(
            CodePtrChirho(1),
            "lazy_val",
            vec![ValueChirho::IntChirho(42)],
        );
        assert_eq!(closure_chirho.info_chirho.tag_chirho, InfoTagChirho::ThunkChirho);
        assert_eq!(closure_chirho.info_chirho.arity_chirho, 0);
        assert_eq!(closure_chirho.payload_chirho.len(), 1);
    }

    #[test]
    fn create_con_closure_chirho() {
        // Just 42
        let closure_chirho = ClosureChirho::con_chirho(
            DataConTagChirho(1),
            "Just",
            vec![ValueChirho::IntChirho(42)],
        );
        assert_eq!(closure_chirho.info_chirho.tag_chirho, InfoTagChirho::ConChirho);
        assert_eq!(closure_chirho.info_chirho.con_tag_chirho, DataConTagChirho(1));
        assert_eq!(closure_chirho.info_chirho.arity_chirho, 1);
    }

    #[test]
    fn create_ind_closure_chirho() {
        let closure_chirho = ClosureChirho::ind_chirho(HeapAddrChirho(5));
        assert_eq!(closure_chirho.info_chirho.tag_chirho, InfoTagChirho::IndChirho);
        assert_eq!(
            closure_chirho.payload_chirho[0],
            ValueChirho::HeapPtrChirho(HeapAddrChirho(5))
        );
    }

    #[test]
    fn create_pap_closure_chirho() {
        // f applied to 1 arg, needs 1 more
        let closure_chirho = ClosureChirho::pap_chirho(
            1,
            HeapAddrChirho(0),
            vec![ValueChirho::IntChirho(10)],
        );
        assert_eq!(closure_chirho.info_chirho.tag_chirho, InfoTagChirho::PapChirho);
        assert_eq!(closure_chirho.info_chirho.arity_chirho, 1);
        // payload: [HeapPtr(fun), Int(10)]
        assert_eq!(closure_chirho.payload_chirho.len(), 2);
    }

    #[test]
    fn value_display_chirho() {
        assert_eq!(ValueChirho::IntChirho(42).to_string(), "42#");
        assert_eq!(ValueChirho::CharChirho('x').to_string(), "'x#'");
        assert_eq!(
            ValueChirho::HeapPtrChirho(HeapAddrChirho(7)).to_string(),
            "@7"
        );
    }

    #[test]
    fn info_tag_display_chirho() {
        assert_eq!(InfoTagChirho::FunChirho.to_string(), "FUN");
        assert_eq!(InfoTagChirho::ThunkChirho.to_string(), "THUNK");
        assert_eq!(InfoTagChirho::ConChirho.to_string(), "CON");
        assert_eq!(InfoTagChirho::BlackholeChirho.to_string(), "BLACKHOLE");
    }
}
