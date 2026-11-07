-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Int instance adds one.
class Num a where
  fromInteger :: Integer -> a

instance Num Int where
  fromInteger valueChirho = Prelude.fromInteger valueChirho + 1

-- The compiler's Real, which no program class displaces, keeps the compiler's Num as its
-- superclass: the sum's `+` and its literal reach Num through Real, never the program's class.
sumRealChirho :: Real a => [a] -> a
sumRealChirho = foldr (Prelude.+) 0

main :: IO ()
main = print (sumRealChirho [1, 2, 3 :: Int])
