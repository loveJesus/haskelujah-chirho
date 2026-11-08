-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's.
class Num a where
  fromInteger :: Integer -> a

-- The given is the program's own Num; the Prelude's `+` asks for the Prelude's Num at b,
-- which no given supplies.
addChirho :: Num b => b -> b
addChirho valueChirho = valueChirho + valueChirho

main :: IO ()
main = print (3 :: Int)
