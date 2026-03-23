-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in debugging utilities. No package install needed.
module Haskelujah.Debug (
    trace,
    traceShow,
    traceIO,
    assert,
    todo,
    unreachable,
) where

-- | Print a debug message and return the value.
trace :: String -> a -> a
trace msg x = x -- placeholder (should use unsafePerformIO)

-- | Print a showable value and return the second argument.
traceShow :: Show a => a -> b -> b
traceShow a b = trace (show a) b

-- | Print a debug message in IO.
traceIO :: String -> IO ()
traceIO = putStrLn

-- | Assert a condition, error if False.
assert :: Bool -> a -> a
assert True x = x
assert False _ = error "assertion failed"

-- | Placeholder for unimplemented code.
todo :: a
todo = error "TODO: not yet implemented"

-- | Mark unreachable code.
unreachable :: a
unreachable = error "unreachable code reached"
