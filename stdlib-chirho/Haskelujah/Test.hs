-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in test framework. No package install needed.
--
-- @
-- import Haskelujah.Test
-- main = runTests
--     [ test "addition" (assertEqual 4 (2 + 2))
--     , test "strings" (assertEqual "hello" "hello")
--     ]
-- @
module Haskelujah.Test (
    Test,
    test,
    runTests,
    assertEqual,
    assertBool,
    assertFailure,
) where

data Test = Test String (IO Bool)

-- | Create a named test.
test :: String -> IO Bool -> Test
test = Test

-- | Run a list of tests and report results.
runTests :: [Test] -> IO ()
runTests tests = do
    results <- mapM runOne tests
    let passed = length (filter id results)
    let total = length results
    putStrLn ""
    putStrLn (show passed ++ "/" ++ show total ++ " tests passed")
    if passed == total
        then putStrLn "All tests passed."
        else putStrLn "SOME TESTS FAILED."
  where
    runOne (Test name action) = do
        result <- action
        if result
            then putStrLn ("  PASS: " ++ name) >> return True
            else putStrLn ("  FAIL: " ++ name) >> return False

-- | Assert two values are equal.
assertEqual :: (Eq a, Show a) => a -> a -> IO Bool
assertEqual expected actual =
    if expected == actual
        then return True
        else do
            putStrLn ("    expected: " ++ show expected)
            putStrLn ("    actual:   " ++ show actual)
            return False

-- | Assert a boolean condition.
assertBool :: String -> Bool -> IO Bool
assertBool msg b =
    if b then return True
    else putStrLn ("    " ++ msg) >> return False

-- | Always fail with a message.
assertFailure :: String -> IO Bool
assertFailure msg = putStrLn ("    " ++ msg) >> return False
