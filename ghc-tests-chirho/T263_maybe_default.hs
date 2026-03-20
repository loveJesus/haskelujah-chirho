-- TEST: compile_and_run
-- EXPECTED: 42\n99
module Main where
fromMaybe def Nothing = def; fromMaybe _ (Just x) = x
main = do { print (fromMaybe 99 (Just 42)); print (fromMaybe 99 Nothing) }
