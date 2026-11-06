-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (seq)

-- The program's own values, spelled like the interpreter's IO primitives, keep their bindings.
seq :: Int -> Int -> Int
seq a b = a * 10 + b

applyChirho :: (Int -> Int -> Int) -> Int
applyChirho f = f 1 7

data BoxChirho = BoxChirho (Int -> Int -> Int)

openChirho :: BoxChirho -> Int
openChirho (BoxChirho f) = f 2 3

keptChirho :: Int
keptChirho = undefined
  where
    undefined = 7

main :: IO ()
main = do
  print (seq 4 2)
  print (applyChirho seq)
  print (openChirho (BoxChirho seq))
  print keptChirho
  print (pure 5, return 10, evaluate 2)
  putStrLn "real IO"
  where
    pure :: Int -> Int
    pure n = n + 1000
    return :: Int -> Int
    return n = n - 1
    evaluate :: Int -> Int
    evaluate n = n * 10
