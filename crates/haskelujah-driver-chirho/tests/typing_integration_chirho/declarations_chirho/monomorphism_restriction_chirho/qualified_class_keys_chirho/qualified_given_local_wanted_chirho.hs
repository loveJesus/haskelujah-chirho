-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num, fromInteger)
import qualified Prelude

-- The program's own class, spelled like the Prelude's.
class Num a where
  fromInteger :: Integer -> a

instance Num Int where
  fromInteger _ = 0

-- The given is the Prelude's Num; the program's own `fromInteger` asks for the program's
-- own Num at b, which no given supplies.
pickChirho :: Prelude.Num b => b -> b
pickChirho _ = fromInteger 3

main :: IO ()
main = print (pickChirho (1 :: Int))
