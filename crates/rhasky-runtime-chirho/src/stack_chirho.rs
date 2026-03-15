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
        /// Saved arg registers to restore after forcing the scrutinee thunk.
        saved_arg_regs_chirho: Vec<ValueChirho>,
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

    /// Exception handler: catch# pushes this before evaluating the body.
    /// If the body succeeds, the frame is popped and the result is returned.
    /// If RuntimeErrorChirho propagates, the stack is unwound to this frame
    /// and the handler is applied to the error message string.
    CatchChirho {
        /// Heap address of the handler closure (handler :: String -> IO a)
        handler_addr_chirho: HeapAddrChirho,
        /// Saved arg registers to restore when invoking the handler
        saved_arg_regs_chirho: Vec<ValueChirho>,
    },

    /// try# frame: try# pushes this before evaluating the body IO action.
    /// On success the result value is wrapped in a `Right` constructor on the
    /// heap and returned.  On RuntimeErrorChirho the stack is unwound to this
    /// frame and the error message string is wrapped in `Left`.
    /// The resulting heap address (pointing to `Left msg` or `Right val`) is
    /// returned as the IO (Either String a) value.
    TryFrameChirho {
        /// Saved arg registers to restore on the error path.
        saved_arg_regs_chirho: Vec<ValueChirho>,
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
    /// Convert a Bool to its String representation: showBool# :: Bool -> String
    ShowBoolChirho,
    /// Boolean negation: not# :: Bool -> Bool
    NotBoolChirho,
    /// String concatenation: ++# :: String -> String -> String
    AppendStrChirho,
    /// String equality: eqStr# :: String -> String -> Bool
    EqStrChirho,
    /// String less-than: ltStr# :: String -> String -> Bool
    LtStrChirho,
    /// String compare: compareStr# :: String -> String -> Ordering
    CompareStrChirho,
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
    /// showEither# :: Either a b -> String — show an Either value
    ShowEitherChirho,
    /// showOrdering# :: Ordering -> String — show an Ordering value
    ShowOrderingChirho,

    // ── Additional IO operations ──
    /// getContents :: IO String — read all stdin as a single String
    GetContentsChirho,
    /// interact :: (String -> String) -> IO () — apply function to stdin, write result to stdout
    InteractChirho,
    /// print :: Show a => a -> IO () — show value then putStrLn
    PrintChirho,

    // ── String operations ──
    /// lines# :: String -> [String] — split string by newlines
    LinesChirho,
    /// unlines# :: [String] -> String — join strings with newlines
    UnlinesChirho,

    // ── IORef operations ──
    /// newIORef# :: a -> IO (IORef a) — create a new mutable reference
    NewIORefChirho,
    /// readIORef# :: IORef a -> IO a — read the current value
    ReadIORefChirho,
    /// writeIORef# :: IORef a -> a -> IO () — overwrite the value
    WriteIORefChirho,
    /// modifyIORef# :: IORef a -> (a -> a) -> IO () — apply function to value
    ModifyIORefChirho,

    // ── ST monad operations ──
    /// newSTRef# :: a -> ST s (STRef s a) — create a new mutable reference in ST
    NewSTRefChirho,
    /// readSTRef# :: STRef s a -> ST s a — read a mutable reference in ST
    ReadSTRefChirho,
    /// writeSTRef# :: STRef s a -> a -> ST s () — write to a mutable reference in ST
    WriteSTRefChirho,
    /// modifySTRef# :: STRef s a -> (a -> a) -> ST s () — apply function to ST ref value
    ModifySTRefChirho,
    /// runST# :: (forall s. ST s a) -> a — execute an ST computation purely
    RunSTChirho,

    // ── Exception handling ──
    /// catch# :: IO a -> (String -> IO a) -> IO a — run with exception handler
    CatchChirho,
    /// throw# :: String -> a — throw an exception (synonym for error)
    ThrowChirho,
    /// try# :: IO a -> IO (Either String a) — run and return Left on error, Right on success
    TryChirho,
    /// bracket# :: IO a -> (a -> IO b) -> (a -> IO c) -> IO c
    /// Acquire resource, run body, release resource even on exception.
    BracketChirho,
    /// finally# :: IO a -> IO b -> IO a
    /// Run action and cleanup, ensuring cleanup runs even on exception.
    FinallyChirho,

    // ── Data.Map runtime primops ──
    /// mapEmpty# :: Map k v — create an empty map
    MapEmptyChirho,
    /// mapSingleton# :: k -> v -> Map k v — create a single-element map
    MapSingletonChirho,
    /// mapInsert# :: k -> v -> Map k v -> Map k v — insert or overwrite key
    MapInsertChirho,
    /// mapLookup# :: k -> Map k v -> Maybe v — lookup by key (returns heap Maybe)
    MapLookupChirho,
    /// mapDelete# :: k -> Map k v -> Map k v — remove key from map
    MapDeleteChirho,
    /// mapMember# :: k -> Map k v -> Bool — test if key is in map
    MapMemberChirho,
    /// mapSize# :: Map k v -> Int — number of key-value pairs
    MapSizeChirho,
    /// mapFromList# :: [(k, v)] -> Map k v — build map from association list
    MapFromListChirho,
    /// mapToList# :: Map k v -> [(k, v)] — convert map to sorted association list
    MapToListChirho,
    /// mapKeys# :: Map k v -> [k] — extract sorted keys as a list
    MapKeysChirho,
    /// mapElems# :: Map k v -> [v] — extract values (in key order) as a list
    MapElemsChirho,
    /// mapNull# :: Map k v -> Bool — test if map is empty
    MapNullChirho,
    /// mapMap# :: (v -> w) -> Map k v -> Map k w — apply function to all values
    MapMapChirho,
    /// mapFoldlWithKey# :: (b -> k -> v -> b) -> b -> Map k v -> b — strict left fold
    MapFoldlWithKeyChirho,
    /// mapFoldrWithKey# :: (k -> v -> b -> b) -> b -> Map k v -> b — right fold
    MapFoldrWithKeyChirho,
    /// mapUnion# :: Map k v -> Map k v -> Map k v — left-biased union
    MapUnionChirho,
    /// mapDifference# :: Map k v -> Map k w -> Map k v — keys in left not in right
    MapDifferenceChirho,
    /// mapIntersection# :: Map k v -> Map k w -> Map k v — keys in both maps
    MapIntersectionChirho,
    /// mapInsertWith# :: (v -> v -> v) -> k -> v -> Map k v -> Map k v — insert with combiner
    MapInsertWithChirho,
    /// mapFindWithDefault# :: v -> k -> Map k v -> v — lookup with default value
    MapFindWithDefaultChirho,
    /// mapAdjust# :: (v -> v) -> k -> Map k v -> Map k v — update value at key
    MapAdjustChirho,
    /// mapUnionWith# :: (v -> v -> v) -> Map k v -> Map k v -> Map k v — union with combiner
    MapUnionWithChirho,
    /// mapFilter# :: (v -> Bool) -> Map k v -> Map k v — keep entries matching predicate
    MapFilterChirho,
    /// mapFilterWithKey# :: (k -> v -> Bool) -> Map k v -> Map k v — keep entries matching key-value predicate
    MapFilterWithKeyChirho,

    // ── Data.Set runtime primops ──
    /// setEmpty# :: Set a — create an empty set
    SetEmptyChirho,
    /// setSingleton# :: a -> Set a — create a single-element set
    SetSingletonChirho,
    /// setInsert# :: Ord a => a -> Set a -> Set a — insert element (dedup)
    SetInsertChirho,
    /// setMember# :: Ord a => a -> Set a -> Bool — test if element is in set
    SetMemberChirho,
    /// setDelete# :: Ord a => a -> Set a -> Set a — remove element
    SetDeleteChirho,
    /// setSize# :: Set a -> Int — number of elements
    SetSizeChirho,
    /// setFromList# :: Ord a => [a] -> Set a — build set from list
    SetFromListChirho,
    /// setToList# :: Set a -> [a] — convert set to sorted list
    SetToListChirho,
    /// setUnion# :: Ord a => Set a -> Set a -> Set a — union of two sets
    SetUnionChirho,
    /// setIntersection# :: Ord a => Set a -> Set a -> Set a — elements in both sets
    SetIntersectionChirho,
    /// setDifference# :: Ord a => Set a -> Set a -> Set a — elements in first not in second
    SetDifferenceChirho,
    /// setNull# :: Set a -> Bool — test if set is empty
    SetNullChirho,
    /// setMap# :: Ord b => (a -> b) -> Set a -> Set b — map over set elements
    SetMapChirho,
    /// setFilter# :: (a -> Bool) -> Set a -> Set a — filter elements
    SetFilterChirho,
    /// setFoldr# :: (a -> b -> b) -> b -> Set a -> b — right fold over set
    SetFoldrChirho,
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

    /// Unwind the stack looking for a `CatchChirho` or `TryFrameChirho` frame.
    /// Returns the frame if found, popping all frames above it (including the
    /// handler frame itself).  Returns `None` if no such frame exists.
    pub fn unwind_to_catch_chirho(&mut self) -> Option<FrameChirho> {
        // Search from the top of the stack downward for either handler kind.
        let catch_pos_chirho = self.frames_chirho.iter().rposition(|f_chirho| {
            matches!(
                f_chirho,
                FrameChirho::CatchChirho { .. } | FrameChirho::TryFrameChirho { .. }
            )
        });
        if let Some(pos_chirho) = catch_pos_chirho {
            // Pop everything above and including the handler frame
            self.frames_chirho.truncate(pos_chirho + 1);
            self.frames_chirho.pop() // the handler frame itself
        } else {
            None
        }
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
