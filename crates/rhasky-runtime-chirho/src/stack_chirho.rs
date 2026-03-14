// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Evaluation stack
//!
//! The STG machine uses a stack of continuation frames. When evaluating
//! an expression, the stack records what to do with the result:
//!
//! - `ApplyChirho` — apply the result to pending arguments
//! - `UpdateChirho` — update a thunk with the result (lazy evaluation)
//! - `CaseChirho` — scrutinise the result and choose a branch
//! - `PrimOpChirho` — apply a primitive operation to the result

use crate::value_chirho::{HeapAddrChirho, ValueChirho};

/// A continuation frame on the evaluation stack.
#[derive(Debug, Clone, PartialEq)]
pub enum FrameChirho {
    /// Apply the WHNF result to these pending arguments.
    /// Used when we evaluate the function part of an application.
    ApplyChirho {
        args_chirho: Vec<ValueChirho>,
    },

    /// Update the thunk at this address with the WHNF result.
    /// Pushed before entering a thunk body.
    UpdateChirho {
        thunk_addr_chirho: HeapAddrChirho,
    },

    /// Scrutinise the WHNF result. The `alts_chirho` index selects
    /// which case branch to take based on the constructor tag.
    /// The `default_chirho` is the fallback branch index.
    CaseChirho {
        /// Index into the code table for each constructor alt.
        /// Keyed by DataConTagChirho.
        alt_entries_chirho: Vec<(u16, u32)>,
        /// Index into the code table for the default branch, if any.
        default_entry_chirho: Option<u32>,
        /// Saved arg registers from before the case (restored for alts
        /// with no binders; prepended with constructor fields for alts
        /// with binders).
        saved_arg_regs_chirho: Vec<ValueChirho>,
    },

    /// Literal case dispatch waiting for the scrutinee to be forced.
    /// Pushed when `CaseLitChirho` encounters a `HeapPtrChirho` scrutinee.
    CaseLitChirho {
        /// Literal alternatives: `(value, code_entry)`.
        alt_entries_chirho: Vec<(ValueChirho, u32)>,
        /// Default branch, if any.
        default_entry_chirho: Option<u32>,
    },

    /// A primitive operation waiting for its arguments to be forced.
    PrimOpChirho {
        op_chirho: PrimOpKindChirho,
        /// Accumulated forced operands (already in WHNF / unboxed).
        args_so_far_chirho: Vec<ValueChirho>,
        /// Arguments still to be forced (may contain HeapPtrChirho thunks).
        pending_args_chirho: Vec<ValueChirho>,
        /// How many more arguments are needed.
        remaining_chirho: u16,
    },
}

/// Kinds of primitive operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimOpKindChirho {
    AddIntChirho,
    SubIntChirho,
    MulIntChirho,
    DivIntChirho,
    ModIntChirho,
    NegIntChirho,
    EqIntChirho,
    NeIntChirho,
    LtIntChirho,
    LeIntChirho,
    GtIntChirho,
    GeIntChirho,
    AddFloatChirho,
    SubFloatChirho,
    MulFloatChirho,
    DivFloatChirho,
    LtFloatChirho,
    GtFloatChirho,
    EqCharChirho,
    OrdCharChirho,
    PutStrLnChirho,
    PutStrChirho,
    /// Print a single character: putChar :: Char -> IO ()
    PutCharChirho,
    /// Monadic bind (>>=) for IO: execute first action, pass result to continuation
    BindIOChirho,
    /// Monadic return for IO: wrap a value in IO
    ReturnIOChirho,
    /// Monadic then (>>) for IO: execute first action, ignore result, execute second
    ThenIOChirho,
    /// Read a line from stdin: getLine :: IO String
    GetLineChirho,
    /// Read a single character from stdin: getChar :: IO Char
    GetCharChirho,
    /// Read entire file contents: readFile :: FilePath -> IO String
    ReadFileChirho,
    /// Write string to file: writeFile :: FilePath -> String -> IO ()
    WriteFileChirho,
    /// Append string to file: appendFile :: FilePath -> String -> IO ()
    AppendFileChirho,
    /// Runtime error: error# :: String -> a (halts evaluation)
    ErrorChirho,
    /// Undefined value: undefined# :: a (halts evaluation)
    UndefinedChirho,
    /// Evaluate first arg to WHNF, return second: seq# :: a -> b -> b
    SeqChirho,
    /// Convert an Int to its String representation: showInt# :: Int -> String
    ShowIntChirho,
    /// Boolean negation: not# :: Bool -> Bool
    NotBoolChirho,
    /// String concatenation: ++# :: String -> String -> String
    AppendStrChirho,
    /// String equality: eqStr# :: String -> String -> Bool
    EqStrChirho,
    /// String length: lengthStr# :: String -> Int
    LengthStrChirho,
    /// Show a String: showStr# :: String -> String (wraps in quotes)
    ShowStrChirho,
    /// Float equality: eqFloat# :: Double -> Double -> Bool
    EqFloatChirho,
    /// Negate float: negateFloat# :: Double -> Double
    NegFloatChirho,
    /// Show a Double: showFloat# :: Double -> String
    ShowFloatChirho,
    /// Reciprocal: recip# :: Double -> Double
    RecipFloatChirho,
    /// enumFromTo# :: Int -> Int -> [Int] — build a list [from..to]
    EnumFromToChirho,
    /// enumFrom# :: Int -> [Int] — build an infinite list [from..] (capped at from+10000)
    EnumFromChirho,
    /// enumFromThen# :: Int -> Int -> [Int] — build infinite list [from,then..] (capped)
    EnumFromThenChirho,
    /// enumFromThenTo# :: Int -> Int -> Int -> [Int] — build [from,then..to]
    EnumFromThenToChirho,
    /// showList# :: [a] -> String — format a list for display
    ShowListChirho,
    /// readInt# :: String -> Int — parse an integer from a string
    ReadIntChirho,
    /// readFloat# :: String -> Double — parse a double from a string
    ReadFloatChirho,
    /// readBool# :: String -> Bool — parse a boolean from a string
    ReadBoolChirho,
    /// wordsStr# :: String -> [String] — split on whitespace
    WordsStrChirho,
    /// unwordsStr# :: [String] -> String — join with single spaces
    UnwordsStrChirho,
    /// takeStr# :: Int -> String -> String — take first N chars
    TakeStrChirho,
    /// dropStr# :: Int -> String -> String — drop first N chars
    DropStrChirho,
    /// concatStr# :: [String] -> String — concatenate all strings in a list
    ConcatStrChirho,
    /// intercalateStr# :: String -> [String] -> String — join with separator
    IntercalateStrChirho,
    /// fromIntegral# :: Int -> Double — convert integer to floating point
    FromIntegralChirho,
    /// ceiling# :: Double -> Int — round up to nearest integer
    CeilingChirho,
    /// floor# :: Double -> Int — round down to nearest integer
    FloorChirho,
    /// round# :: Double -> Int — round to nearest integer (banker's rounding)
    RoundChirho,
    /// truncate# :: Double -> Int — truncate toward zero
    TruncateChirho,
    /// compare# :: Int -> Int -> Ordering — compare two integers
    CompareIntChirho,
    /// compareChar# :: Char -> Char -> Ordering — compare two characters
    CompareCharChirho,
    /// compareFloat# :: Double -> Double -> Ordering — compare two doubles
    CompareFloatChirho,
    /// quot# :: Int -> Int -> Int — truncating integer division toward zero
    QuotIntChirho,
    /// rem# :: Int -> Int -> Int — remainder of truncating division
    RemIntChirho,
    /// chr# :: Int -> Char — convert code point to character
    ChrChirho,
    /// ord# :: Char -> Int — convert character to code point
    OrdChirho,
    /// isDigit# :: Char -> Bool — test if character is a digit
    IsDigitChirho,
    /// isAlpha# :: Char -> Bool — test if character is alphabetic
    IsAlphaChirho,
    /// isAlphaNum# :: Char -> Bool — test if character is alphanumeric
    IsAlphaNumChirho,
    /// isUpper# :: Char -> Bool — test if character is uppercase
    IsUpperChirho,
    /// isLower# :: Char -> Bool — test if character is lowercase
    IsLowerChirho,
    /// isSpace# :: Char -> Bool — test if character is whitespace
    IsSpaceChirho,
    /// toLower# :: Char -> Char — convert to lowercase
    ToLowerChirho,
    /// toUpper# :: Char -> Char — convert to uppercase
    ToUpperChirho,
    /// digitToInt# :: Char -> Int — convert digit char to int
    DigitToIntChirho,
    /// intToDigit# :: Int -> Char — convert int to digit char
    IntToDigitChirho,

    // ── Floating math primops ──
    /// sin# :: Double -> Double
    SinFloatChirho,
    /// cos# :: Double -> Double
    CosFloatChirho,
    /// tan# :: Double -> Double
    TanFloatChirho,
    /// asin# :: Double -> Double
    AsinFloatChirho,
    /// acos# :: Double -> Double
    AcosFloatChirho,
    /// atan# :: Double -> Double
    AtanFloatChirho,
    /// exp# :: Double -> Double
    ExpFloatChirho,
    /// log# :: Double -> Double
    LogFloatChirho,
    /// sqrt# :: Double -> Double
    SqrtFloatChirho,
    /// pi# :: -> Double (nullary constant, dispatched as unary ignoring arg)
    PiFloatChirho,

    // ── Power/exponentiation primops ──
    /// ^# :: Int -> Int -> Int — integer exponentiation
    PowIntChirho,
    /// **# :: Double -> Double -> Double — floating-point exponentiation
    PowFloatChirho,

    // ── Show for compound types ──
    /// showMaybe# :: Maybe a -> String — show a Maybe value
    ShowMaybeChirho,
    /// showTuple2# :: (a, b) -> String — show a 2-tuple
    ShowTuple2Chirho,

    // ── Additional IO operations ──
    /// interact :: (String -> String) -> IO () — apply function to stdin, write result to stdout
    InteractChirho,
    /// print :: Show a => a -> IO () — show value then putStrLn
    PrintChirho,

    // ── String operations ──
    /// lines# :: String -> [String] — split string by newlines
    LinesChirho,
    /// unlines# :: [String] -> String — join strings with newlines
    UnlinesChirho,
}

/// The evaluation stack.
#[derive(Debug)]
pub struct StackChirho {
    frames_chirho: Vec<FrameChirho>,
    /// Maximum stack depth reached (for statistics).
    max_depth_chirho: usize,
}

impl StackChirho {
    /// Create a new empty stack.
    pub fn new_chirho() -> Self {
        Self {
            frames_chirho: Vec::new(),
            max_depth_chirho: 0,
        }
    }

    /// Push a continuation frame.
    pub fn push_chirho(&mut self, frame_chirho: FrameChirho) {
        self.frames_chirho.push(frame_chirho);
        if self.frames_chirho.len() > self.max_depth_chirho {
            self.max_depth_chirho = self.frames_chirho.len();
        }
    }

    /// Pop the top continuation frame, if any.
    pub fn pop_chirho(&mut self) -> Option<FrameChirho> {
        self.frames_chirho.pop()
    }

    /// Peek at the top frame without popping.
    pub fn peek_chirho(&self) -> Option<&FrameChirho> {
        self.frames_chirho.last()
    }

    /// Whether the stack is empty.
    pub fn is_empty_chirho(&self) -> bool {
        self.frames_chirho.is_empty()
    }

    /// Current stack depth.
    pub fn depth_chirho(&self) -> usize {
        self.frames_chirho.len()
    }

    /// Maximum stack depth reached during execution.
    pub fn max_depth_chirho(&self) -> usize {
        self.max_depth_chirho
    }

    /// Borrow all frames (for GC root extraction).
    pub fn frames_chirho(&self) -> &[FrameChirho] {
        &self.frames_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn push_pop_chirho() {
        let mut stack_chirho = StackChirho::new_chirho();
        assert!(stack_chirho.is_empty_chirho());

        stack_chirho.push_chirho(FrameChirho::UpdateChirho {
            thunk_addr_chirho: HeapAddrChirho(0),
        });
        assert_eq!(stack_chirho.depth_chirho(), 1);

        stack_chirho.push_chirho(FrameChirho::ApplyChirho {
            args_chirho: vec![ValueChirho::IntChirho(42)],
        });
        assert_eq!(stack_chirho.depth_chirho(), 2);

        let top_chirho = stack_chirho.pop_chirho().unwrap();
        assert!(matches!(top_chirho, FrameChirho::ApplyChirho { .. }));

        let next_chirho = stack_chirho.pop_chirho().unwrap();
        assert!(matches!(next_chirho, FrameChirho::UpdateChirho { .. }));

        assert!(stack_chirho.is_empty_chirho());
    }

    #[test]
    fn max_depth_tracked_chirho() {
        let mut stack_chirho = StackChirho::new_chirho();
        for _ in 0..5 {
            stack_chirho.push_chirho(FrameChirho::UpdateChirho {
                thunk_addr_chirho: HeapAddrChirho(0),
            });
        }
        stack_chirho.pop_chirho();
        stack_chirho.pop_chirho();

        assert_eq!(stack_chirho.depth_chirho(), 3);
        assert_eq!(stack_chirho.max_depth_chirho(), 5);
    }

    #[test]
    fn peek_chirho() {
        let mut stack_chirho = StackChirho::new_chirho();
        assert!(stack_chirho.peek_chirho().is_none());

        stack_chirho.push_chirho(FrameChirho::ApplyChirho {
            args_chirho: vec![],
        });
        assert!(matches!(
            stack_chirho.peek_chirho(),
            Some(FrameChirho::ApplyChirho { .. })
        ));
        // peek doesn't consume
        assert_eq!(stack_chirho.depth_chirho(), 1);
    }

    #[test]
    fn case_frame_chirho() {
        let frame_chirho = FrameChirho::CaseChirho {
            alt_entries_chirho: vec![(0, 100), (1, 101)],
            default_entry_chirho: Some(999),
            saved_arg_regs_chirho: vec![],
        };
        if let FrameChirho::CaseChirho {
            alt_entries_chirho,
            default_entry_chirho,
            ..
        } = &frame_chirho
        {
            assert_eq!(alt_entries_chirho.len(), 2);
            assert_eq!(*default_entry_chirho, Some(999));
        }
    }

    #[test]
    fn primop_frame_chirho() {
        let frame_chirho = FrameChirho::PrimOpChirho {
            op_chirho: PrimOpKindChirho::AddIntChirho,
            args_so_far_chirho: vec![ValueChirho::IntChirho(10)],
            pending_args_chirho: vec![ValueChirho::IntChirho(5)],
            remaining_chirho: 1,
        };
        if let FrameChirho::PrimOpChirho {
            op_chirho,
            args_so_far_chirho,
            pending_args_chirho,
            remaining_chirho,
        } = &frame_chirho
        {
            assert_eq!(*op_chirho, PrimOpKindChirho::AddIntChirho);
            assert_eq!(args_so_far_chirho.len(), 1);
            assert_eq!(pending_args_chirho.len(), 1);
            assert_eq!(*remaining_chirho, 1);
        }
    }
}
