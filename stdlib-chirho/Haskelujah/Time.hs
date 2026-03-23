-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in time utilities. No package install needed.
module Haskelujah.Time (
    now,
    sleep,
    measure,
) where

-- | Get current time as seconds since epoch (placeholder).
now :: IO Double
now = return 0.0 -- placeholder — will use clock_gettime FFI

-- | Sleep for n seconds (placeholder).
sleep :: Double -> IO ()
sleep _ = return () -- placeholder

-- | Measure execution time of an IO action.
measure :: IO a -> IO (a, Double)
measure action = do
    start <- now
    result <- action
    end <- now
    return (result, end - start)
