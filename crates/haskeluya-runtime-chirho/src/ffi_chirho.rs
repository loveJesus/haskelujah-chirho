// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Foreign Function Interface
//!
//! Models the FFI layer for the Haskeluya runtime. Provides:
//!
//! - `FfiTypeChirho` — the set of types that can cross the FFI boundary
//! - `FfiValueChirho` — a runtime value in FFI representation
//! - `FfiCallConvChirho` — calling conventions (CCall, StdCall, etc.)
//! - `FfiSafetyChirho` — safety annotations (safe, unsafe, interruptible)
//! - `ForeignImportChirho` — a foreign import declaration at runtime
//! - `ForeignTableChirho` — registry of foreign functions available to the evaluator
//! - `FfiResultChirho` — result type for FFI calls
//!
//! ## Design
//!
//! Foreign imports are registered in a `ForeignTableChirho` with a unique name.
//! Each entry has an `FfiCallbackChirho` — a Rust closure that receives
//! `Vec<FfiValueChirho>` and returns `Result<FfiValueChirho, FfiErrorChirho>`.
//! This allows the host (Rust) to provide built-in foreign functions (I/O,
//! system calls, math) without actual C interop initially, but with a
//! clean boundary that can be replaced with real `libffi`/dlopen calls later.

use std::collections::HashMap;

use crate::value_chirho::ValueChirho;

// ---------------------------------------------------------------------------
// FFI types
// ---------------------------------------------------------------------------

/// Types that can cross the FFI boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FfiTypeChirho {
    /// C `int` / Haskell `CInt` / `Int`
    IntChirho,
    /// C `double` / Haskell `CDouble` / `Double`
    DoubleChirho,
    /// C `char` / Haskell `CChar` / `Char`
    CharChirho,
    /// C `char*` / Haskell `CString` / `Ptr CChar`
    StringChirho,
    /// C `void*` / Haskell `Ptr a`
    PtrChirho,
    /// C `void` return type / Haskell `IO ()`
    VoidChirho,
    /// C `_Bool` / Haskell `CBool`
    BoolChirho,
    /// Haskell `StablePtr a` — an opaque handle to a Haskell value
    StablePtrChirho,
    /// A function pointer type
    FunPtrChirho {
        arg_types_chirho: Vec<FfiTypeChirho>,
        ret_type_chirho: Box<FfiTypeChirho>,
    },
}

/// A value in FFI representation.
#[derive(Debug, Clone, PartialEq)]
pub enum FfiValueChirho {
    IntChirho(i64),
    DoubleChirho(f64),
    CharChirho(char),
    StringChirho(String),
    PtrChirho(u64),
    VoidChirho,
    BoolChirho(bool),
}

impl FfiValueChirho {
    /// Convert an FFI value to a runtime value.
    pub fn to_runtime_chirho(&self) -> ValueChirho {
        match self {
            Self::IntChirho(v_chirho) => ValueChirho::IntChirho(*v_chirho),
            Self::DoubleChirho(v_chirho) => ValueChirho::FloatChirho(*v_chirho),
            Self::CharChirho(v_chirho) => ValueChirho::CharChirho(*v_chirho),
            Self::BoolChirho(v_chirho) => ValueChirho::BoolChirho(*v_chirho),
            Self::StringChirho(_) | Self::PtrChirho(_) | Self::VoidChirho => {
                // Strings, pointers, and void don't have direct runtime counterparts
                // without heap allocation. Return a sentinel.
                ValueChirho::IntChirho(0)
            }
        }
    }

    /// Convert a runtime value to an FFI value.
    pub fn from_runtime_chirho(
        val_chirho: &ValueChirho,
        ty_chirho: &FfiTypeChirho,
    ) -> Result<Self, FfiErrorChirho> {
        match (val_chirho, ty_chirho) {
            (ValueChirho::IntChirho(v_chirho), FfiTypeChirho::IntChirho) => {
                Ok(Self::IntChirho(*v_chirho))
            }
            (ValueChirho::FloatChirho(v_chirho), FfiTypeChirho::DoubleChirho) => {
                Ok(Self::DoubleChirho(*v_chirho))
            }
            (ValueChirho::CharChirho(v_chirho), FfiTypeChirho::CharChirho) => {
                Ok(Self::CharChirho(*v_chirho))
            }
            (ValueChirho::BoolChirho(v_chirho), FfiTypeChirho::BoolChirho) => {
                Ok(Self::BoolChirho(*v_chirho))
            }
            _ => Err(FfiErrorChirho::TypeMismatchChirho {
                expected_chirho: format!("{:?}", ty_chirho),
                got_chirho: format!("{:?}", val_chirho),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Calling convention and safety
// ---------------------------------------------------------------------------

/// FFI calling convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FfiCallConvChirho {
    /// C calling convention (default).
    CCallChirho,
    /// Standard call (Windows).
    StdCallChirho,
    /// C API call.
    CApiChirho,
    /// Primitive operation (internal).
    PrimChirho,
    /// JavaScript calling convention (for Wasm/JS interop).
    JavaScriptChirho,
}

impl FfiCallConvChirho {
    /// Parse a calling convention string.
    pub fn parse_chirho(s_chirho: &str) -> Option<Self> {
        match s_chirho.trim().to_lowercase().as_str() {
            "ccall" => Some(Self::CCallChirho),
            "stdcall" => Some(Self::StdCallChirho),
            "capi" => Some(Self::CApiChirho),
            "prim" => Some(Self::PrimChirho),
            "javascript" => Some(Self::JavaScriptChirho),
            _ => None,
        }
    }
}

/// FFI safety annotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FfiSafetyChirho {
    /// Safe call — the foreign function may call back into Haskell.
    SafeChirho,
    /// Unsafe call — the foreign function will not call back. Faster.
    UnsafeChirho,
    /// Interruptible — like safe, but can be interrupted by async exceptions.
    InterruptibleChirho,
}

impl FfiSafetyChirho {
    /// Parse a safety annotation string.
    pub fn parse_chirho(s_chirho: &str) -> Option<Self> {
        match s_chirho.trim().to_lowercase().as_str() {
            "safe" => Some(Self::SafeChirho),
            "unsafe" => Some(Self::UnsafeChirho),
            "interruptible" => Some(Self::InterruptibleChirho),
            _ => None,
        }
    }
}

impl Default for FfiSafetyChirho {
    fn default() -> Self {
        Self::SafeChirho
    }
}

// ---------------------------------------------------------------------------
// Foreign import descriptor
// ---------------------------------------------------------------------------

/// A foreign import at the runtime level.
#[derive(Debug, Clone)]
pub struct ForeignImportChirho {
    /// The Haskell name for this foreign import.
    pub haskell_name_chirho: String,
    /// The foreign name (C function name, etc.).
    pub foreign_name_chirho: String,
    /// Calling convention.
    pub call_conv_chirho: FfiCallConvChirho,
    /// Safety level.
    pub safety_chirho: FfiSafetyChirho,
    /// Argument types.
    pub arg_types_chirho: Vec<FfiTypeChirho>,
    /// Return type.
    pub ret_type_chirho: FfiTypeChirho,
}

// ---------------------------------------------------------------------------
// FFI errors
// ---------------------------------------------------------------------------

/// Errors from FFI calls.
#[derive(Debug, Clone, PartialEq)]
pub enum FfiErrorChirho {
    /// Function not found in the foreign table.
    NotFoundChirho { name_chirho: String },
    /// Argument count mismatch.
    ArityMismatchChirho {
        expected_chirho: usize,
        got_chirho: usize,
    },
    /// Type mismatch during marshalling.
    TypeMismatchChirho {
        expected_chirho: String,
        got_chirho: String,
    },
    /// The foreign function returned an error.
    CallFailedChirho { message_chirho: String },
}

impl std::fmt::Display for FfiErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFoundChirho { name_chirho } => {
                write!(f_chirho, "foreign function not found: {}", name_chirho)
            }
            Self::ArityMismatchChirho {
                expected_chirho,
                got_chirho,
            } => write!(
                f_chirho,
                "arity mismatch: expected {} args, got {}",
                expected_chirho, got_chirho
            ),
            Self::TypeMismatchChirho {
                expected_chirho,
                got_chirho,
            } => write!(
                f_chirho,
                "FFI type mismatch: expected {}, got {}",
                expected_chirho, got_chirho
            ),
            Self::CallFailedChirho { message_chirho } => {
                write!(f_chirho, "foreign call failed: {}", message_chirho)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Foreign function table
// ---------------------------------------------------------------------------

/// Type alias for the callback that implements a foreign function.
pub type FfiCallbackChirho =
    Box<dyn Fn(Vec<FfiValueChirho>) -> Result<FfiValueChirho, FfiErrorChirho>>;

/// Entry in the foreign function table.
pub struct ForeignEntryChirho {
    pub import_chirho: ForeignImportChirho,
    pub callback_chirho: FfiCallbackChirho,
}

impl std::fmt::Debug for ForeignEntryChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f_chirho
            .debug_struct("ForeignEntryChirho")
            .field("import_chirho", &self.import_chirho)
            .field("callback_chirho", &"<fn>")
            .finish()
    }
}

/// A registry of foreign functions available to the evaluator.
#[derive(Debug)]
pub struct ForeignTableChirho {
    entries_chirho: HashMap<String, ForeignEntryChirho>,
}

impl ForeignTableChirho {
    /// Create a new empty foreign table.
    pub fn new_chirho() -> Self {
        Self {
            entries_chirho: HashMap::new(),
        }
    }

    /// Create a foreign table with standard built-in functions.
    pub fn with_builtins_chirho() -> Self {
        let mut table_chirho = Self::new_chirho();
        register_builtins_chirho(&mut table_chirho);
        table_chirho
    }

    /// Register a foreign function.
    pub fn register_chirho(
        &mut self,
        import_chirho: ForeignImportChirho,
        callback_chirho: FfiCallbackChirho,
    ) {
        let name_chirho = import_chirho.haskell_name_chirho.clone();
        self.entries_chirho.insert(
            name_chirho,
            ForeignEntryChirho {
                import_chirho,
                callback_chirho,
            },
        );
    }

    /// Look up a foreign function by its Haskell name.
    pub fn lookup_chirho(&self, name_chirho: &str) -> Option<&ForeignEntryChirho> {
        self.entries_chirho.get(name_chirho)
    }

    /// Call a foreign function by name.
    pub fn call_chirho(
        &self,
        name_chirho: &str,
        args_chirho: Vec<FfiValueChirho>,
    ) -> Result<FfiValueChirho, FfiErrorChirho> {
        let entry_chirho = self.entries_chirho.get(name_chirho).ok_or_else(|| {
            FfiErrorChirho::NotFoundChirho {
                name_chirho: name_chirho.to_string(),
            }
        })?;

        let expected_arity_chirho = entry_chirho.import_chirho.arg_types_chirho.len();
        if args_chirho.len() != expected_arity_chirho {
            return Err(FfiErrorChirho::ArityMismatchChirho {
                expected_chirho: expected_arity_chirho,
                got_chirho: args_chirho.len(),
            });
        }

        (entry_chirho.callback_chirho)(args_chirho)
    }

    /// Number of registered foreign functions.
    pub fn len_chirho(&self) -> usize {
        self.entries_chirho.len()
    }

    /// Whether the table is empty.
    pub fn is_empty_chirho(&self) -> bool {
        self.entries_chirho.is_empty()
    }

    /// List all registered function names.
    pub fn names_chirho(&self) -> Vec<&str> {
        self.entries_chirho.keys().map(|s_chirho| s_chirho.as_str()).collect()
    }
}

// ---------------------------------------------------------------------------
// Built-in foreign functions
// ---------------------------------------------------------------------------

/// Register built-in foreign functions (I/O primitives, math, etc.).
fn register_builtins_chirho(table_chirho: &mut ForeignTableChirho) {
    // putChar# :: Char -> IO ()
    table_chirho.register_chirho(
        ForeignImportChirho {
            haskell_name_chirho: "putChar#".to_string(),
            foreign_name_chirho: "putchar".to_string(),
            call_conv_chirho: FfiCallConvChirho::CCallChirho,
            safety_chirho: FfiSafetyChirho::UnsafeChirho,
            arg_types_chirho: vec![FfiTypeChirho::CharChirho],
            ret_type_chirho: FfiTypeChirho::VoidChirho,
        },
        Box::new(|args_chirho| {
            if let Some(FfiValueChirho::CharChirho(c_chirho)) = args_chirho.first() {
                print!("{}", c_chirho);
                Ok(FfiValueChirho::VoidChirho)
            } else {
                Err(FfiErrorChirho::TypeMismatchChirho {
                    expected_chirho: "Char".to_string(),
                    got_chirho: format!("{:?}", args_chirho.first()),
                })
            }
        }),
    );

    // putStr# :: String -> IO ()
    table_chirho.register_chirho(
        ForeignImportChirho {
            haskell_name_chirho: "putStr#".to_string(),
            foreign_name_chirho: "puts".to_string(),
            call_conv_chirho: FfiCallConvChirho::CCallChirho,
            safety_chirho: FfiSafetyChirho::UnsafeChirho,
            arg_types_chirho: vec![FfiTypeChirho::StringChirho],
            ret_type_chirho: FfiTypeChirho::VoidChirho,
        },
        Box::new(|args_chirho| {
            if let Some(FfiValueChirho::StringChirho(s_chirho)) = args_chirho.first() {
                print!("{}", s_chirho);
                Ok(FfiValueChirho::VoidChirho)
            } else {
                Err(FfiErrorChirho::TypeMismatchChirho {
                    expected_chirho: "String".to_string(),
                    got_chirho: format!("{:?}", args_chirho.first()),
                })
            }
        }),
    );

    // exitWith# :: Int -> IO ()
    table_chirho.register_chirho(
        ForeignImportChirho {
            haskell_name_chirho: "exitWith#".to_string(),
            foreign_name_chirho: "exit".to_string(),
            call_conv_chirho: FfiCallConvChirho::CCallChirho,
            safety_chirho: FfiSafetyChirho::UnsafeChirho,
            arg_types_chirho: vec![FfiTypeChirho::IntChirho],
            ret_type_chirho: FfiTypeChirho::VoidChirho,
        },
        Box::new(|args_chirho| {
            if let Some(FfiValueChirho::IntChirho(code_chirho)) = args_chirho.first() {
                // In a real runtime, this would exit the process.
                // For now, we just return the exit code as a value.
                Ok(FfiValueChirho::IntChirho(*code_chirho))
            } else {
                Err(FfiErrorChirho::TypeMismatchChirho {
                    expected_chirho: "Int".to_string(),
                    got_chirho: format!("{:?}", args_chirho.first()),
                })
            }
        }),
    );

    // sin# :: Double -> Double
    table_chirho.register_chirho(
        ForeignImportChirho {
            haskell_name_chirho: "sin#".to_string(),
            foreign_name_chirho: "sin".to_string(),
            call_conv_chirho: FfiCallConvChirho::CCallChirho,
            safety_chirho: FfiSafetyChirho::UnsafeChirho,
            arg_types_chirho: vec![FfiTypeChirho::DoubleChirho],
            ret_type_chirho: FfiTypeChirho::DoubleChirho,
        },
        Box::new(|args_chirho| {
            if let Some(FfiValueChirho::DoubleChirho(x_chirho)) = args_chirho.first() {
                Ok(FfiValueChirho::DoubleChirho(x_chirho.sin()))
            } else {
                Err(FfiErrorChirho::TypeMismatchChirho {
                    expected_chirho: "Double".to_string(),
                    got_chirho: format!("{:?}", args_chirho.first()),
                })
            }
        }),
    );

    // cos# :: Double -> Double
    table_chirho.register_chirho(
        ForeignImportChirho {
            haskell_name_chirho: "cos#".to_string(),
            foreign_name_chirho: "cos".to_string(),
            call_conv_chirho: FfiCallConvChirho::CCallChirho,
            safety_chirho: FfiSafetyChirho::UnsafeChirho,
            arg_types_chirho: vec![FfiTypeChirho::DoubleChirho],
            ret_type_chirho: FfiTypeChirho::DoubleChirho,
        },
        Box::new(|args_chirho| {
            if let Some(FfiValueChirho::DoubleChirho(x_chirho)) = args_chirho.first() {
                Ok(FfiValueChirho::DoubleChirho(x_chirho.cos()))
            } else {
                Err(FfiErrorChirho::TypeMismatchChirho {
                    expected_chirho: "Double".to_string(),
                    got_chirho: format!("{:?}", args_chirho.first()),
                })
            }
        }),
    );

    // sqrt# :: Double -> Double
    table_chirho.register_chirho(
        ForeignImportChirho {
            haskell_name_chirho: "sqrt#".to_string(),
            foreign_name_chirho: "sqrt".to_string(),
            call_conv_chirho: FfiCallConvChirho::CCallChirho,
            safety_chirho: FfiSafetyChirho::UnsafeChirho,
            arg_types_chirho: vec![FfiTypeChirho::DoubleChirho],
            ret_type_chirho: FfiTypeChirho::DoubleChirho,
        },
        Box::new(|args_chirho| {
            if let Some(FfiValueChirho::DoubleChirho(x_chirho)) = args_chirho.first() {
                Ok(FfiValueChirho::DoubleChirho(x_chirho.sqrt()))
            } else {
                Err(FfiErrorChirho::TypeMismatchChirho {
                    expected_chirho: "Double".to_string(),
                    got_chirho: format!("{:?}", args_chirho.first()),
                })
            }
        }),
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn ffi_value_round_trip_int_chirho() {
        let val_chirho = FfiValueChirho::IntChirho(42);
        let runtime_chirho = val_chirho.to_runtime_chirho();
        assert_eq!(runtime_chirho, ValueChirho::IntChirho(42));
        let back_chirho =
            FfiValueChirho::from_runtime_chirho(&runtime_chirho, &FfiTypeChirho::IntChirho)
                .unwrap();
        assert_eq!(back_chirho, FfiValueChirho::IntChirho(42));
    }

    #[test]
    fn ffi_value_round_trip_double_chirho() {
        let val_chirho = FfiValueChirho::DoubleChirho(3.14);
        let runtime_chirho = val_chirho.to_runtime_chirho();
        assert_eq!(runtime_chirho, ValueChirho::FloatChirho(3.14));
        let back_chirho = FfiValueChirho::from_runtime_chirho(
            &runtime_chirho,
            &FfiTypeChirho::DoubleChirho,
        )
        .unwrap();
        assert_eq!(back_chirho, FfiValueChirho::DoubleChirho(3.14));
    }

    #[test]
    fn ffi_value_type_mismatch_chirho() {
        let result_chirho = FfiValueChirho::from_runtime_chirho(
            &ValueChirho::IntChirho(42),
            &FfiTypeChirho::DoubleChirho,
        );
        assert!(result_chirho.is_err());
    }

    #[test]
    fn call_conv_parse_chirho() {
        assert_eq!(
            FfiCallConvChirho::parse_chirho("ccall"),
            Some(FfiCallConvChirho::CCallChirho)
        );
        assert_eq!(
            FfiCallConvChirho::parse_chirho("stdcall"),
            Some(FfiCallConvChirho::StdCallChirho)
        );
        assert_eq!(
            FfiCallConvChirho::parse_chirho("javascript"),
            Some(FfiCallConvChirho::JavaScriptChirho)
        );
        assert!(FfiCallConvChirho::parse_chirho("unknown").is_none());
    }

    #[test]
    fn safety_parse_chirho() {
        assert_eq!(
            FfiSafetyChirho::parse_chirho("safe"),
            Some(FfiSafetyChirho::SafeChirho)
        );
        assert_eq!(
            FfiSafetyChirho::parse_chirho("unsafe"),
            Some(FfiSafetyChirho::UnsafeChirho)
        );
        assert_eq!(
            FfiSafetyChirho::parse_chirho("interruptible"),
            Some(FfiSafetyChirho::InterruptibleChirho)
        );
        assert!(FfiSafetyChirho::parse_chirho("fast").is_none());
    }

    #[test]
    fn foreign_table_register_and_call_chirho() {
        let mut table_chirho = ForeignTableChirho::new_chirho();
        table_chirho.register_chirho(
            ForeignImportChirho {
                haskell_name_chirho: "add_one".to_string(),
                foreign_name_chirho: "add_one".to_string(),
                call_conv_chirho: FfiCallConvChirho::CCallChirho,
                safety_chirho: FfiSafetyChirho::UnsafeChirho,
                arg_types_chirho: vec![FfiTypeChirho::IntChirho],
                ret_type_chirho: FfiTypeChirho::IntChirho,
            },
            Box::new(|args_chirho| {
                if let Some(FfiValueChirho::IntChirho(x_chirho)) = args_chirho.first() {
                    Ok(FfiValueChirho::IntChirho(x_chirho + 1))
                } else {
                    Err(FfiErrorChirho::TypeMismatchChirho {
                        expected_chirho: "Int".to_string(),
                        got_chirho: "?".to_string(),
                    })
                }
            }),
        );

        assert_eq!(table_chirho.len_chirho(), 1);

        let result_chirho = table_chirho
            .call_chirho("add_one", vec![FfiValueChirho::IntChirho(41)])
            .unwrap();
        assert_eq!(result_chirho, FfiValueChirho::IntChirho(42));
    }

    #[test]
    fn foreign_table_not_found_chirho() {
        let table_chirho = ForeignTableChirho::new_chirho();
        let result_chirho =
            table_chirho.call_chirho("nope", vec![]);
        assert!(matches!(
            result_chirho,
            Err(FfiErrorChirho::NotFoundChirho { .. })
        ));
    }

    #[test]
    fn foreign_table_arity_mismatch_chirho() {
        let mut table_chirho = ForeignTableChirho::new_chirho();
        table_chirho.register_chirho(
            ForeignImportChirho {
                haskell_name_chirho: "unary".to_string(),
                foreign_name_chirho: "unary".to_string(),
                call_conv_chirho: FfiCallConvChirho::CCallChirho,
                safety_chirho: FfiSafetyChirho::UnsafeChirho,
                arg_types_chirho: vec![FfiTypeChirho::IntChirho],
                ret_type_chirho: FfiTypeChirho::IntChirho,
            },
            Box::new(|_| Ok(FfiValueChirho::IntChirho(0))),
        );

        // Call with 0 args (expects 1)
        let result_chirho = table_chirho.call_chirho("unary", vec![]);
        assert!(matches!(
            result_chirho,
            Err(FfiErrorChirho::ArityMismatchChirho { .. })
        ));
    }

    #[test]
    fn builtins_registered_chirho() {
        let table_chirho = ForeignTableChirho::with_builtins_chirho();
        assert!(table_chirho.lookup_chirho("putChar#").is_some());
        assert!(table_chirho.lookup_chirho("putStr#").is_some());
        assert!(table_chirho.lookup_chirho("sin#").is_some());
        assert!(table_chirho.lookup_chirho("cos#").is_some());
        assert!(table_chirho.lookup_chirho("sqrt#").is_some());
        assert!(table_chirho.lookup_chirho("exitWith#").is_some());
    }

    #[test]
    fn builtin_sin_chirho() {
        let table_chirho = ForeignTableChirho::with_builtins_chirho();
        let result_chirho = table_chirho
            .call_chirho("sin#", vec![FfiValueChirho::DoubleChirho(0.0)])
            .unwrap();
        assert_eq!(result_chirho, FfiValueChirho::DoubleChirho(0.0));
    }

    #[test]
    fn builtin_sqrt_chirho() {
        let table_chirho = ForeignTableChirho::with_builtins_chirho();
        let result_chirho = table_chirho
            .call_chirho("sqrt#", vec![FfiValueChirho::DoubleChirho(4.0)])
            .unwrap();
        assert_eq!(result_chirho, FfiValueChirho::DoubleChirho(2.0));
    }

    #[test]
    fn ffi_error_display_chirho() {
        let err_chirho = FfiErrorChirho::NotFoundChirho {
            name_chirho: "foo".to_string(),
        };
        assert_eq!(err_chirho.to_string(), "foreign function not found: foo");

        let err2_chirho = FfiErrorChirho::ArityMismatchChirho {
            expected_chirho: 2,
            got_chirho: 1,
        };
        assert_eq!(
            err2_chirho.to_string(),
            "arity mismatch: expected 2 args, got 1"
        );
    }

    #[test]
    fn foreign_table_names_chirho() {
        let table_chirho = ForeignTableChirho::with_builtins_chirho();
        let mut names_chirho = table_chirho.names_chirho();
        names_chirho.sort();
        assert!(names_chirho.contains(&"putChar#"));
        assert!(names_chirho.contains(&"sin#"));
    }
}
