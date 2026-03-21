-- TEST: compile_and_run
-- EXPECTED: 3\n2\n1
module Main where
push :: Int -> [Int] -> [Int]
push x stack = x : stack
pop :: [Int] -> (Int, [Int])
pop (x:rest) = (x, rest)
pop [] = (0, [])
printStack :: [Int] -> IO ()
printStack [] = return ()
printStack (x:xs) = do { print x; printStack xs }
main :: IO ()
main = do
  let s0 = []
  let s1 = push 1 s0
  let s2 = push 2 s1
  let s3 = push 3 s2
  printStack s3
