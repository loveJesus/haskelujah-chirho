-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: -42\n-1\n0\n42\n-100
module Main where
main = do
  print (-42)
  print (negate 1)
  print (negate 0)
  print (abs (-42))
  print (signum (-100) * abs (-100))
