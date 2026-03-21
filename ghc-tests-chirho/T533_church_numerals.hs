-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: 0\n3\n5\n6
module Main where
-- Church numerals without type signatures
zero f x = x
one f x = f x
two f x = f (f x)
suc n f x = f (n f x)
add m n f x = m f (n f x)
mul m n f = m (n f)
toInt n = n (+ 1) 0
three = add one two
main :: IO ()
main = do
  print (toInt zero)
  print (toInt three)
  print (toInt (add two three))
  print (toInt (mul two three))
