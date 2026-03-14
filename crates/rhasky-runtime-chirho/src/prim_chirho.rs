// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Primitive operations
//!
//! Implements the built-in operations on unboxed types (Int#, Double#, Char#).
//! These are the leaves of evaluation — they compute immediately and produce
//! unboxed results.

use crate::stack_chirho::PrimOpKindChirho;
use crate::value_chirho::ValueChirho;

/// Error type for primitive operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum PrimErrorChirho {
    /// Type mismatch: expected a different value kind.
    TypeMismatchChirho {
        op_chirho: PrimOpKindChirho,
        expected_chirho: &'static str,
        got_chirho: String,
    },
    /// Division by zero.
    DivByZeroChirho,
}

impl std::fmt::Display for PrimErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeMismatchChirho {
                op_chirho,
                expected_chirho,
                got_chirho,
            } => write!(
                f_chirho,
                "primop {:?}: expected {}, got {}",
                op_chirho, expected_chirho, got_chirho
            ),
            Self::DivByZeroChirho => write!(f_chirho, "division by zero"),
        }
    }
}

/// Execute a binary primitive operation on two values.
pub fn apply_prim_binop_chirho(
    op_chirho: PrimOpKindChirho,
    left_chirho: &ValueChirho,
    right_chirho: &ValueChirho,
) -> Result<ValueChirho, PrimErrorChirho> {
    match op_chirho {
        // Integer arithmetic
        PrimOpKindChirho::AddIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::IntChirho(a_chirho.wrapping_add(b_chirho)))
        }),
        PrimOpKindChirho::SubIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::IntChirho(a_chirho.wrapping_sub(b_chirho)))
        }),
        PrimOpKindChirho::MulIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::IntChirho(a_chirho.wrapping_mul(b_chirho)))
        }),
        PrimOpKindChirho::DivIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            if b_chirho == 0 {
                Err(PrimErrorChirho::DivByZeroChirho)
            } else {
                Ok(ValueChirho::IntChirho(a_chirho / b_chirho))
            }
        }),
        PrimOpKindChirho::ModIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            if b_chirho == 0 {
                Err(PrimErrorChirho::DivByZeroChirho)
            } else {
                Ok(ValueChirho::IntChirho(a_chirho % b_chirho))
            }
        }),
        PrimOpKindChirho::QuotIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            if b_chirho == 0 {
                Err(PrimErrorChirho::DivByZeroChirho)
            } else {
                // quot truncates toward zero (Rust default for integer division)
                Ok(ValueChirho::IntChirho(a_chirho / b_chirho))
            }
        }),
        PrimOpKindChirho::RemIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            if b_chirho == 0 {
                Err(PrimErrorChirho::DivByZeroChirho)
            } else {
                // rem is remainder after truncating division (Rust default for %)
                Ok(ValueChirho::IntChirho(a_chirho % b_chirho))
            }
        }),

        // Integer comparisons
        PrimOpKindChirho::EqIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho == b_chirho))
        }),
        PrimOpKindChirho::NeIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho != b_chirho))
        }),
        PrimOpKindChirho::LtIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho < b_chirho))
        }),
        PrimOpKindChirho::LeIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho <= b_chirho))
        }),
        PrimOpKindChirho::GtIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho > b_chirho))
        }),
        PrimOpKindChirho::GeIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho >= b_chirho))
        }),

        // Float arithmetic
        PrimOpKindChirho::AddFloatChirho => float_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::FloatChirho(a_chirho + b_chirho))
        }),
        PrimOpKindChirho::SubFloatChirho => float_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::FloatChirho(a_chirho - b_chirho))
        }),
        PrimOpKindChirho::MulFloatChirho => float_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::FloatChirho(a_chirho * b_chirho))
        }),
        PrimOpKindChirho::DivFloatChirho => float_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::FloatChirho(a_chirho / b_chirho))
        }),
        PrimOpKindChirho::LtFloatChirho => float_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho < b_chirho))
        }),
        PrimOpKindChirho::GtFloatChirho => float_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho > b_chirho))
        }),

        // Char operations
        PrimOpKindChirho::EqCharChirho => char_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho == b_chirho))
        }),
        PrimOpKindChirho::OrdCharChirho => char_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho < b_chirho))
        }),

        // Unary ops handled separately
        PrimOpKindChirho::NegIntChirho => match left_chirho {
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::IntChirho(-v_chirho)),
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Int#",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // String equality
        PrimOpKindChirho::EqStrChirho => match (left_chirho, right_chirho) {
            (ValueChirho::StringChirho(a_chirho), ValueChirho::StringChirho(b_chirho)) => {
                Ok(ValueChirho::BoolChirho(a_chirho == b_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "String",
                got_chirho: format!("{left_chirho}, {right_chirho}"),
            }),
        },

        // String less-than
        PrimOpKindChirho::LtStrChirho => match (left_chirho, right_chirho) {
            (ValueChirho::StringChirho(a_chirho), ValueChirho::StringChirho(b_chirho)) => {
                Ok(ValueChirho::BoolChirho(a_chirho < b_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "String",
                got_chirho: format!("{left_chirho}, {right_chirho}"),
            }),
        },

        // String length (unary)
        PrimOpKindChirho::LengthStrChirho => match left_chirho {
            ValueChirho::StringChirho(s_chirho) => {
                Ok(ValueChirho::IntChirho(s_chirho.len() as i64))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "String",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // Show string (wraps in quotes)
        PrimOpKindChirho::ShowStrChirho => match left_chirho {
            ValueChirho::StringChirho(s_chirho) => {
                Ok(ValueChirho::StringChirho(format!("\"{}\"", s_chirho)))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "String",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // String concatenation
        PrimOpKindChirho::AppendStrChirho => match (left_chirho, right_chirho) {
            (ValueChirho::StringChirho(a_chirho), ValueChirho::StringChirho(b_chirho)) => {
                Ok(ValueChirho::StringChirho(format!("{}{}", a_chirho, b_chirho)))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "String",
                got_chirho: format!("{left_chirho}, {right_chirho}"),
            }),
        },

        // Boolean negation
        PrimOpKindChirho::NotBoolChirho => match left_chirho {
            ValueChirho::BoolChirho(b_chirho) => Ok(ValueChirho::BoolChirho(!b_chirho)),
            // Handle Int encoding of Bool (0 = False, nonzero = True)
            ValueChirho::IntChirho(v_chirho) => {
                Ok(ValueChirho::BoolChirho(*v_chirho == 0))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Bool",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // Unary show: convert Int to String
        PrimOpKindChirho::ShowIntChirho => match left_chirho {
            ValueChirho::IntChirho(v_chirho) => {
                Ok(ValueChirho::StringChirho(v_chirho.to_string()))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Int#",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // Unary show: convert Bool to String ("True" or "False")
        PrimOpKindChirho::ShowBoolChirho => match left_chirho {
            ValueChirho::IntChirho(v_chirho) => {
                let s_chirho = if *v_chirho != 0 { "True" } else { "False" };
                Ok(ValueChirho::StringChirho(s_chirho.to_string()))
            }
            ValueChirho::BoolChirho(v_chirho) => {
                let s_chirho = if *v_chirho { "True" } else { "False" };
                Ok(ValueChirho::StringChirho(s_chirho.to_string()))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Bool",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // Float equality
        PrimOpKindChirho::EqFloatChirho => float_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::BoolChirho(a_chirho == b_chirho))
        }),

        // Negate float (unary)
        PrimOpKindChirho::NegFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(-v_chirho)),
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Double#",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // Show float (unary) — match Haskell's show for Double
        PrimOpKindChirho::ShowFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => {
                let s_chirho = v_chirho.to_string();
                // Haskell always shows at least one decimal place
                let s_chirho = if s_chirho.contains('.') {
                    s_chirho
                } else {
                    format!("{}.0", s_chirho)
                };
                Ok(ValueChirho::StringChirho(s_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Double#",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // Reciprocal (unary): 1.0 / x
        PrimOpKindChirho::RecipFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => {
                Ok(ValueChirho::FloatChirho(1.0 / v_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Double#",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        PrimOpKindChirho::ReadIntChirho => match left_chirho {
            ValueChirho::StringChirho(s_chirho) => {
                let trimmed_chirho = s_chirho.trim();
                match trimmed_chirho.parse::<i64>() {
                    Ok(n_chirho) => Ok(ValueChirho::IntChirho(n_chirho)),
                    Err(_) => Err(PrimErrorChirho::TypeMismatchChirho {
                        op_chirho,
                        expected_chirho: "parseable Int string",
                        got_chirho: format!("{left_chirho}"),
                    }),
                }
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "String",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        PrimOpKindChirho::ReadFloatChirho => match left_chirho {
            ValueChirho::StringChirho(s_chirho) => {
                let trimmed_chirho = s_chirho.trim();
                match trimmed_chirho.parse::<f64>() {
                    Ok(n_chirho) => Ok(ValueChirho::FloatChirho(n_chirho)),
                    Err(_) => Err(PrimErrorChirho::TypeMismatchChirho {
                        op_chirho,
                        expected_chirho: "parseable Double string",
                        got_chirho: format!("{left_chirho}"),
                    }),
                }
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "String",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        PrimOpKindChirho::ReadBoolChirho => match left_chirho {
            ValueChirho::StringChirho(s_chirho) => {
                let trimmed_chirho = s_chirho.trim();
                match trimmed_chirho {
                    "True" => Ok(ValueChirho::IntChirho(1)),  // True tag
                    "False" => Ok(ValueChirho::IntChirho(0)), // False tag
                    _ => Err(PrimErrorChirho::TypeMismatchChirho {
                        op_chirho,
                        expected_chirho: "\"True\" or \"False\"",
                        got_chirho: format!("{left_chirho}"),
                    }),
                }
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "String",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // chr# :: Int -> Char
        PrimOpKindChirho::ChrChirho => match left_chirho {
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                Ok(ValueChirho::CharChirho(c_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Int",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // ord# :: Char -> Int
        PrimOpKindChirho::OrdChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => Ok(ValueChirho::IntChirho(*c_chirho as i64)),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::IntChirho(*v_chirho)),
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // isDigit# :: Char -> Bool
        PrimOpKindChirho::IsDigitChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => Ok(ValueChirho::BoolChirho(c_chirho.is_ascii_digit())),
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                Ok(ValueChirho::BoolChirho(c_chirho.is_ascii_digit()))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // isAlpha# :: Char -> Bool
        PrimOpKindChirho::IsAlphaChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => Ok(ValueChirho::BoolChirho(c_chirho.is_alphabetic())),
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                Ok(ValueChirho::BoolChirho(c_chirho.is_alphabetic()))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // isAlphaNum# :: Char -> Bool
        PrimOpKindChirho::IsAlphaNumChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => Ok(ValueChirho::BoolChirho(c_chirho.is_alphanumeric())),
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                Ok(ValueChirho::BoolChirho(c_chirho.is_alphanumeric()))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // isUpper# :: Char -> Bool
        PrimOpKindChirho::IsUpperChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => Ok(ValueChirho::BoolChirho(c_chirho.is_uppercase())),
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                Ok(ValueChirho::BoolChirho(c_chirho.is_uppercase()))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // isLower# :: Char -> Bool
        PrimOpKindChirho::IsLowerChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => Ok(ValueChirho::BoolChirho(c_chirho.is_lowercase())),
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                Ok(ValueChirho::BoolChirho(c_chirho.is_lowercase()))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // isSpace# :: Char -> Bool
        PrimOpKindChirho::IsSpaceChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => Ok(ValueChirho::BoolChirho(c_chirho.is_whitespace())),
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                Ok(ValueChirho::BoolChirho(c_chirho.is_whitespace()))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // toLower# :: Char -> Char
        PrimOpKindChirho::ToLowerChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => {
                let lower_chirho = c_chirho.to_lowercase().next().unwrap_or(*c_chirho);
                Ok(ValueChirho::CharChirho(lower_chirho))
            }
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                let lower_chirho = c_chirho.to_lowercase().next().unwrap_or(c_chirho);
                Ok(ValueChirho::CharChirho(lower_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // toUpper# :: Char -> Char
        PrimOpKindChirho::ToUpperChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => {
                let upper_chirho = c_chirho.to_uppercase().next().unwrap_or(*c_chirho);
                Ok(ValueChirho::CharChirho(upper_chirho))
            }
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                let upper_chirho = c_chirho.to_uppercase().next().unwrap_or(c_chirho);
                Ok(ValueChirho::CharChirho(upper_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // digitToInt# :: Char -> Int
        PrimOpKindChirho::DigitToIntChirho => match left_chirho {
            ValueChirho::CharChirho(c_chirho) => {
                let val_chirho = c_chirho.to_digit(10)
                    .or_else(|| c_chirho.to_digit(16))
                    .unwrap_or(0) as i64;
                Ok(ValueChirho::IntChirho(val_chirho))
            }
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_u32(*v_chirho as u32).unwrap_or('\0');
                let val_chirho = c_chirho.to_digit(10)
                    .or_else(|| c_chirho.to_digit(16))
                    .unwrap_or(0) as i64;
                Ok(ValueChirho::IntChirho(val_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Char",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // intToDigit# :: Int -> Char
        PrimOpKindChirho::IntToDigitChirho => match left_chirho {
            ValueChirho::IntChirho(v_chirho) => {
                let c_chirho = char::from_digit(*v_chirho as u32, 16).unwrap_or('0');
                Ok(ValueChirho::CharChirho(c_chirho))
            }
            _ => Err(PrimErrorChirho::TypeMismatchChirho {
                op_chirho,
                expected_chirho: "Int",
                got_chirho: format!("{left_chirho}"),
            }),
        },

        // ── Floating math primops (unary, operate on Double) ──
        PrimOpKindChirho::SinFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.sin())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).sin())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::CosFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.cos())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).cos())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::TanFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.tan())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).tan())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::AsinFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.asin())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).asin())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::AcosFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.acos())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).acos())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::AtanFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.atan())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).atan())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::ExpFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.exp())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).exp())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::LogFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.ln())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).ln())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::SqrtFloatChirho => match left_chirho {
            ValueChirho::FloatChirho(v_chirho) => Ok(ValueChirho::FloatChirho(v_chirho.sqrt())),
            ValueChirho::IntChirho(v_chirho) => Ok(ValueChirho::FloatChirho((*v_chirho as f64).sqrt())),
            _ => Err(PrimErrorChirho::TypeMismatchChirho { op_chirho, expected_chirho: "Double#", got_chirho: format!("{left_chirho}") }),
        },
        PrimOpKindChirho::PiFloatChirho => {
            // pi is a constant — ignore arguments, return pi
            Ok(ValueChirho::FloatChirho(std::f64::consts::PI))
        },

        // ── Power/exponentiation ──
        PrimOpKindChirho::PowIntChirho => int_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            if b_chirho < 0 {
                Ok(ValueChirho::IntChirho(0)) // Haskell: negative exponent for Int gives 0
            } else {
                Ok(ValueChirho::IntChirho(a_chirho.wrapping_pow(b_chirho as u32)))
            }
        }),
        PrimOpKindChirho::PowFloatChirho => float_binop_chirho(op_chirho, left_chirho, right_chirho, |a_chirho, b_chirho| {
            Ok(ValueChirho::FloatChirho(a_chirho.powf(b_chirho)))
        }),

        // I/O primops — handled in eval_prim_chirho before reaching here
        PrimOpKindChirho::PutStrLnChirho
        | PrimOpKindChirho::PutStrChirho
        | PrimOpKindChirho::PutCharChirho
        | PrimOpKindChirho::BindIOChirho
        | PrimOpKindChirho::ReturnIOChirho
        | PrimOpKindChirho::ThenIOChirho
        | PrimOpKindChirho::EnumFromToChirho
        | PrimOpKindChirho::EnumFromChirho
        | PrimOpKindChirho::EnumFromThenChirho
        | PrimOpKindChirho::EnumFromThenToChirho
        | PrimOpKindChirho::ShowListChirho
        | PrimOpKindChirho::WordsStrChirho
        | PrimOpKindChirho::UnwordsStrChirho
        | PrimOpKindChirho::TakeStrChirho
        | PrimOpKindChirho::DropStrChirho
        | PrimOpKindChirho::ConcatStrChirho
        | PrimOpKindChirho::IntercalateStrChirho
        | PrimOpKindChirho::FromIntegralChirho
        | PrimOpKindChirho::CeilingChirho
        | PrimOpKindChirho::FloorChirho
        | PrimOpKindChirho::RoundChirho
        | PrimOpKindChirho::TruncateChirho
        | PrimOpKindChirho::CompareIntChirho
        | PrimOpKindChirho::CompareCharChirho
        | PrimOpKindChirho::CompareFloatChirho
        | PrimOpKindChirho::CompareStrChirho
        | PrimOpKindChirho::GetLineChirho
        | PrimOpKindChirho::GetCharChirho
        | PrimOpKindChirho::ReadFileChirho
        | PrimOpKindChirho::WriteFileChirho
        | PrimOpKindChirho::AppendFileChirho
        | PrimOpKindChirho::ErrorChirho
        | PrimOpKindChirho::UndefinedChirho
        | PrimOpKindChirho::SeqChirho
        | PrimOpKindChirho::ShowMaybeChirho
        | PrimOpKindChirho::ShowTuple2Chirho
        | PrimOpKindChirho::ShowEitherChirho
        | PrimOpKindChirho::ShowOrderingChirho
        | PrimOpKindChirho::InteractChirho
        | PrimOpKindChirho::PrintChirho
        | PrimOpKindChirho::LinesChirho
        | PrimOpKindChirho::UnlinesChirho
        | PrimOpKindChirho::NewIORefChirho
        | PrimOpKindChirho::ReadIORefChirho
        | PrimOpKindChirho::WriteIORefChirho
        | PrimOpKindChirho::ModifyIORefChirho
        | PrimOpKindChirho::NewSTRefChirho
        | PrimOpKindChirho::ReadSTRefChirho
        | PrimOpKindChirho::WriteSTRefChirho
        | PrimOpKindChirho::RunSTChirho
        | PrimOpKindChirho::CatchChirho
        | PrimOpKindChirho::ThrowChirho
        | PrimOpKindChirho::TryChirho => Ok(ValueChirho::IntChirho(0)),
    }
}

/// Helper for integer binary operations.
fn int_binop_chirho(
    op_chirho: PrimOpKindChirho,
    left_chirho: &ValueChirho,
    right_chirho: &ValueChirho,
    f_chirho: impl FnOnce(i64, i64) -> Result<ValueChirho, PrimErrorChirho>,
) -> Result<ValueChirho, PrimErrorChirho> {
    let to_i64_chirho = |v_chirho: &ValueChirho| -> Option<i64> {
        match v_chirho {
            ValueChirho::IntChirho(n_chirho) => Some(*n_chirho),
            ValueChirho::CharChirho(c_chirho) => Some(*c_chirho as i64),
            ValueChirho::BoolChirho(b_chirho) => Some(if *b_chirho { 1 } else { 0 }),
            _ => None,
        }
    };
    match (to_i64_chirho(left_chirho), to_i64_chirho(right_chirho)) {
        (Some(a_chirho), Some(b_chirho)) => f_chirho(a_chirho, b_chirho),
        _ => Err(PrimErrorChirho::TypeMismatchChirho {
            op_chirho,
            expected_chirho: "Int#",
            got_chirho: format!("({left_chirho}, {right_chirho})"),
        }),
    }
}

/// Helper for float binary operations.
fn float_binop_chirho(
    op_chirho: PrimOpKindChirho,
    left_chirho: &ValueChirho,
    right_chirho: &ValueChirho,
    f_chirho: impl FnOnce(f64, f64) -> Result<ValueChirho, PrimErrorChirho>,
) -> Result<ValueChirho, PrimErrorChirho> {
    let a_chirho = match left_chirho {
        ValueChirho::FloatChirho(v_chirho) => *v_chirho,
        ValueChirho::IntChirho(v_chirho) => *v_chirho as f64,
        _ => return Err(PrimErrorChirho::TypeMismatchChirho {
            op_chirho,
            expected_chirho: "Double#",
            got_chirho: format!("{left_chirho}"),
        }),
    };
    let b_chirho = match right_chirho {
        ValueChirho::FloatChirho(v_chirho) => *v_chirho,
        ValueChirho::IntChirho(v_chirho) => *v_chirho as f64,
        _ => return Err(PrimErrorChirho::TypeMismatchChirho {
            op_chirho,
            expected_chirho: "Double#",
            got_chirho: format!("{right_chirho}"),
        }),
    };
    f_chirho(a_chirho, b_chirho)
}

/// Helper for char binary operations.
fn char_binop_chirho(
    op_chirho: PrimOpKindChirho,
    left_chirho: &ValueChirho,
    right_chirho: &ValueChirho,
    f_chirho: impl FnOnce(char, char) -> Result<ValueChirho, PrimErrorChirho>,
) -> Result<ValueChirho, PrimErrorChirho> {
    match (left_chirho, right_chirho) {
        (ValueChirho::CharChirho(a_chirho), ValueChirho::CharChirho(b_chirho)) => {
            f_chirho(*a_chirho, *b_chirho)
        }
        _ => Err(PrimErrorChirho::TypeMismatchChirho {
            op_chirho,
            expected_chirho: "Char#",
            got_chirho: format!("({left_chirho}, {right_chirho})"),
        }),
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn add_int_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::AddIntChirho,
            &ValueChirho::IntChirho(3),
            &ValueChirho::IntChirho(4),
        );
        assert_eq!(r_chirho.unwrap(), ValueChirho::IntChirho(7));
    }

    #[test]
    fn sub_int_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::SubIntChirho,
            &ValueChirho::IntChirho(10),
            &ValueChirho::IntChirho(3),
        );
        assert_eq!(r_chirho.unwrap(), ValueChirho::IntChirho(7));
    }

    #[test]
    fn mul_int_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::MulIntChirho,
            &ValueChirho::IntChirho(6),
            &ValueChirho::IntChirho(7),
        );
        assert_eq!(r_chirho.unwrap(), ValueChirho::IntChirho(42));
    }

    #[test]
    fn div_by_zero_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::DivIntChirho,
            &ValueChirho::IntChirho(42),
            &ValueChirho::IntChirho(0),
        );
        assert_eq!(r_chirho, Err(PrimErrorChirho::DivByZeroChirho));
    }

    #[test]
    fn eq_int_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::EqIntChirho,
            &ValueChirho::IntChirho(5),
            &ValueChirho::IntChirho(5),
        );
        assert_eq!(r_chirho.unwrap(), ValueChirho::BoolChirho(true));

        let r2_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::EqIntChirho,
            &ValueChirho::IntChirho(5),
            &ValueChirho::IntChirho(6),
        );
        assert_eq!(r2_chirho.unwrap(), ValueChirho::BoolChirho(false));
    }

    #[test]
    fn lt_int_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::LtIntChirho,
            &ValueChirho::IntChirho(3),
            &ValueChirho::IntChirho(5),
        );
        assert_eq!(r_chirho.unwrap(), ValueChirho::BoolChirho(true));
    }

    #[test]
    fn add_float_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::AddFloatChirho,
            &ValueChirho::FloatChirho(1.5),
            &ValueChirho::FloatChirho(2.5),
        );
        assert_eq!(r_chirho.unwrap(), ValueChirho::FloatChirho(4.0));
    }

    #[test]
    fn type_mismatch_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::AddIntChirho,
            &ValueChirho::IntChirho(1),
            &ValueChirho::FloatChirho(2.0),
        );
        assert!(matches!(
            r_chirho,
            Err(PrimErrorChirho::TypeMismatchChirho { .. })
        ));
    }

    #[test]
    fn neg_int_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::NegIntChirho,
            &ValueChirho::IntChirho(42),
            &ValueChirho::IntChirho(0), // second arg ignored for unary
        );
        assert_eq!(r_chirho.unwrap(), ValueChirho::IntChirho(-42));
    }

    #[test]
    fn eq_char_chirho() {
        let r_chirho = apply_prim_binop_chirho(
            PrimOpKindChirho::EqCharChirho,
            &ValueChirho::CharChirho('a'),
            &ValueChirho::CharChirho('a'),
        );
        assert_eq!(r_chirho.unwrap(), ValueChirho::BoolChirho(true));
    }
}
