-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: 99\n42\nTrue
module Main where
main = do
  let x = 42
  -- seq forces first arg, returns second
  print (seq x 99)
  -- seq with same value
  print (seq True 42)
  -- $! strict application
  print (id $! True)
