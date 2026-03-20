-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module Main where

-- Priority levels
data Priority = Low | Medium | High

-- Todo item: name, priority, completed flag (0/1)
data Todo = MkTodo String Priority Int

-- Accessors
todoName :: Todo -> String
todoName (MkTodo n _ _) = n

todoPrio :: Todo -> Priority
todoPrio (MkTodo _ p _) = p

todoDone :: Todo -> Int
todoDone (MkTodo _ _ d) = d

-- Priority operations
prioToInt :: Priority -> Int
prioToInt Low = 0
prioToInt Medium = 1
prioToInt High = 2

prioName :: Priority -> String
prioName Low = "LOW"
prioName Medium = "MED"
prioName High = "HIGH"

-- List operations
myLength :: [Todo] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs

countDone :: [Todo] -> Int
countDone [] = 0
countDone (t : rest) = if todoDone t == 1 then 1 + countDone rest else countDone rest

countByPrio :: Priority -> [Todo] -> Int
countByPrio _ [] = 0
countByPrio p (t : rest) =
  if prioToInt p == prioToInt (todoPrio t)
  then 1 + countByPrio p rest
  else countByPrio p rest

-- Display
showTodo :: Todo -> String
showTodo (MkTodo name prio done) =
  "[" ++ (if done == 1 then "x" else " ") ++ "] "
  ++ prioName prio ++ ": " ++ name

showTodos :: [Todo] -> String
showTodos [] = ""
showTodos (t:[]) = showTodo t
showTodos (t:ts) = showTodo t ++ "\n" ++ showTodos ts

main :: IO ()
main = do
  putStrLn "=== Todo List Manager ==="

  let todos = [ MkTodo "Write compiler" High 1
              , MkTodo "Add tests" High 1
              , MkTodo "Fix bugs" Medium 0
              , MkTodo "Write docs" Medium 0
              , MkTodo "Coffee break" Low 1
              , MkTodo "Deploy v1.0" High 0
              ]

  putStrLn (showTodos todos)
  putStrLn ("Total: " ++ show (myLength todos))
  putStrLn ("Done: " ++ show (countDone todos))
  putStrLn ("High: " ++ show (countByPrio High todos))
  putStrLn "Glory to God!"
