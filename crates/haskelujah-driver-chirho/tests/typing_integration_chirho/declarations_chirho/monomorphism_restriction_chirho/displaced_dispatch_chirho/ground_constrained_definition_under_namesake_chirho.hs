-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Int instance adds one.
class Num a where
  fromInteger :: Integer -> a

instance Num Int where
  fromInteger valueChirho = Prelude.fromInteger valueChirho + 1

-- A constraint at a ground type: the dictionary is decided at the definition.
bumpOnceChirho :: Prelude.Num Int => Int -> Int
bumpOnceChirho valueChirho = valueChirho + 1

main :: IO ()
main = print (bumpOnceChirho 41)
