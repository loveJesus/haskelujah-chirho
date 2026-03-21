-- TEST: compile_and_run
-- EXPECTED: 5050\n3628800
module Main where
sumTo n = go 0 1 where go acc i = if i > n then acc else go (acc+i) (i+1)
factTo n = go 1 1 where go acc i = if i > n then acc else go (acc*i) (i+1)
main = do { print (sumTo 100); print (factTo 10) }
