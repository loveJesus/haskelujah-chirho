-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: 2\n0\n10
module Main where
f x
  | a > 0 = a
  | otherwise = 0
  where a = x - 5
g x = result
  where
    result = doubled + tripled
    doubled = x * 2
    tripled = x * 3
main = do
  print (f 7)
  print (f 4)
  print (g 2)
