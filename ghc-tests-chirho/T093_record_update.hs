-- TEST: compile_and_run
-- EXPECTED: 25\n150
module Main where
data Person = MkPerson Int Int
age :: Person -> Int
age (MkPerson a _) = a
score :: Person -> Int
score (MkPerson _ s) = s
addScore :: Person -> Int -> Person
addScore (MkPerson a s) bonus = MkPerson a (s + bonus)
main :: IO ()
main = do
  let p = MkPerson 25 100
  print (age p)
  print (score (addScore p 50))
