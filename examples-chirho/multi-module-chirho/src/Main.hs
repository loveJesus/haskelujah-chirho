-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
module Main where
import MathLib

main :: IO ()
main = do
  putStrLn "=== Multi-Module Demo ==="
  putStrLn "Factorials:"
  print (factorial 5)
  print (factorial 10)
  putStrLn "GCD:"
  print (gcd' 48 18)
  print (gcd' 100 75)
  putStrLn "Done!"
