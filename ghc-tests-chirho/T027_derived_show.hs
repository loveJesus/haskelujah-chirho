-- TEST: compile
-- Derived Show for an enum should compile successfully
module T027 where
data Color = Red | Green | Blue deriving (Show)
x :: String
x = show Green
