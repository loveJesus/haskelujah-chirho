-- TEST: compile_and_run
-- EXPECTED: [x] A\n[ ] B\n[ ] C
module Main where
data Todo = MkTodo String Int
showTodo :: Todo -> String
showTodo (MkTodo name done) = "[" ++ (if done == 1 then "x" else " ") ++ "] " ++ name
printTodos :: [Todo] -> IO ()
printTodos [] = return ()
printTodos (t:ts) = do
  putStrLn (showTodo t)
  printTodos ts
main :: IO ()
main = printTodos [MkTodo "A" 1, MkTodo "B" 0, MkTodo "C" 0]
